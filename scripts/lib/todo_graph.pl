#!/usr/bin/env perl
use strict; use warnings;
my ($cmd, $slug) = @ARGV;
my $root   = $ENV{TODO_ROOT}   or die "TODO_ROOT unset\n";
my $census = $ENV{TODO_CENSUS} // "$root/census.md";

my %folder;   # slug -> folder name (status)
my %needs;    # slug -> [needs]
my %exists;   # slug -> 1 if it is a real ticket OR a census row

# A slug looks like a kebab token with a known area prefix.
my $SLUG = qr/\b((?:engine|core|parse|macro|pipeline|shape|kw|ka|aw|runner|format|canon|tokens|noncanon)-[a-z0-9-]+)\b/;

# 1. ticket files: docs/tickets/<folder>/<slug>.md
for my $f (qw(critical planned maybe wip done)) {
    my $dir = "$root/$f";
    opendir(my $dh, $dir) or next;
    for my $name (readdir $dh) {
        next unless $name =~ /^(.+)\.md$/;
        my $s = $1;
        $folder{$s} = $f; $exists{$s} = 1; $needs{$s} //= [];
        open(my $fh, '<', "$dir/$name") or next;
        local $/; my $txt = <$fh>; close $fh;
        if ($txt =~ /^---\s*\n(.*?)\n---/s) {
            my $fm = $1;
            if ($fm =~ /^needs:\s*\[(.*?)\]/m) {
                my @n = grep { length } map { s/^\s+|\s+$//gr } split /,/, $1;
                $needs{$s} = [@n];
            }
        }
    }
}

# 2. census rows: derive slug from mechanic name + section prefix; needs from the
# machinery cell's slug tokens. Section is tracked by the last "## N. Keyword..." header.
open(my $cf, '<', $census) or die "no census: $census\n";
my $pre = '';   # active census-section prefix, set by the last "## N. <name>" header
while (my $line = <$cf>) {
    chomp $line;                                     # else the trailing field is "\n"
    if ($line =~ /^##\s+\d+\.\s+(.+)$/) {
        my $h = $1;                                  # tolerate suffixes, e.g. "Card shapes (layouts)"
        $pre = $h =~ /Card shapes/       ? 'shape'
             : $h =~ /Keyword abilities/ ? 'kw'
             : $h =~ /Keyword actions/   ? 'ka'
             : $h =~ /Ability words/     ? 'aw'
             : '';
        next;
    }
    next unless $pre;                                # only rows inside a census section
    next unless $line =~ /^\|\s*(.+?)\s*\|/;         # a table row
    my $first = $1;
    next if $first =~ /^-+$/ || $first =~ /^(Keyword|Action|Ability word|Layout)$/; # header/sep
    (my $kebab = lc $first) =~ s/[^a-z0-9]+/-/g; $kebab =~ s/^-|-$//g;
    next unless length $kebab;
    my $s = "$pre-$kebab";
    $exists{$s} //= 1;            # census rows exist as graph nodes
    $folder{$s} //= 'census';    # not a real folder; never "done"
    my @cells = split /\|/, $line;   # trailing empty field dropped by split; last = machinery cell
    my $mach = $cells[-1] // '';
    my @n; while ($mach =~ /$SLUG/g) { push @n, $1; }
    $needs{$s} //= [@n];
}
close $cf;

sub done { ($folder{$_[0]} // '') eq 'done' }
sub triage { my $f = $folder{$_[0]} // ''; $f eq 'critical' || $f eq 'planned' || $f eq 'maybe' }

# readiness: a triage item whose every need exists and is done.
sub ready {
    my $s = shift;
    return 0 unless triage($s);
    for my $n (@{$needs{$s}}) { return 0 unless $exists{$n} && done($n); }
    return 1;
}
sub blocked {
    my $s = shift;
    return 0 unless triage($s);
    for my $n (@{$needs{$s}}) { return 1 if $exists{$n} && !done($n); }
    return 0;
}

my @order = ('critical','planned','maybe','wip','done','census');
my %rank; @rank{@order} = 0..$#order;
sub by_folder { ($rank{$folder{$a}//'census'} <=> $rank{$folder{$b}//'census'}) || ($a cmp $b) }

if ($cmd eq 'ready') {
    print "$_  ($folder{$_})\n" for sort by_folder grep { ready($_) } keys %exists;
}
elsif ($cmd eq 'blocked') {
    for my $s (sort by_folder grep { blocked($_) } keys %exists) {
        my @un = grep { $exists{$_} && !done($_) } @{$needs{$s}};
        print "$s  <- @un\n";
    }
}
elsif ($cmd eq 'needs') {
    print "$_\n" for @{$needs{$slug} // []};
}
elsif ($cmd eq 'graph') {
    die "graph needs a slug\n" unless $slug;
    my %seen; my @up;
    my @stack = @{$needs{$slug} // []};
    while (@stack) { my $n = shift @stack; next if $seen{$n}++; push @up, $n; push @stack, @{$needs{$n} // []}; }
    my @down = grep { my $t = $_; grep { $_ eq $slug } @{$needs{$t}} } keys %exists;
    print "needs (upstream): ", (join(', ', map { "$_ [".($folder{$_}//'?')."]" } @up) || '(none)'), "\n";
    print "blocks (downstream): ", (join(', ', map { "$_ [".($folder{$_}//'?')."]" } sort @down) || '(none)'), "\n";
}
elsif ($cmd eq 'mint') {
    die "mint needs a slug\n" unless $slug;
    my @n = @{$needs{$slug} // []};
    print "---\nneeds: [", join(', ', @n), "]\n---\n";
    print "Minted from the census ($slug). Fill in a real description.\n";
}
elsif ($cmd eq 'iscensus') {
    # exit 0 iff $slug is a census-derived node (not a ticket, not unknown).
    exit(($folder{$slug // ''} // '') eq 'census' ? 0 : 1);
}
elsif ($cmd eq 'check') {
    my @problems;
    # dangling: a need naming no existing node.
    for my $s (sort keys %exists) {
        for my $n (@{$needs{$s}}) { push @problems, "dangling: $s needs unknown $n" unless $exists{$n}; }
    }
    # cycles: DFS colouring.
    my (%color, @cyc);
    my $dfs; $dfs = sub {
        my $u = shift; $color{$u} = 1;
        for my $v (@{$needs{$u} // []}) {
            next unless $exists{$v};
            if (($color{$v}//0) == 1) { push @cyc, "$u -> $v"; }
            elsif (!$color{$v}) { $dfs->($v); }
        }
        $color{$u} = 2;
    };
    $dfs->($_) for grep { !$color{$_} } sort keys %exists;
    push @problems, "cycle: $_" for @cyc;
    if (@problems) { print "FAIL\n"; print "$_\n" for @problems; }
    else { print "OK: no cycles, no dangling needs\n"; }
}
else { die "unknown command: $cmd\n"; }
