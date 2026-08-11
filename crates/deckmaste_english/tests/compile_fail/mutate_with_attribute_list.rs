use deckmaste_english::syntax::WithAttributeList;
use deckmaste_english::syntax::WithAttributeMember;

fn bypass_checked_list(list: &mut WithAttributeList, first: WithAttributeMember) {
    list.first = first;
}

fn main() {}
