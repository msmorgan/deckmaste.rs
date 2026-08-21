use deckmaste_construction::constructions;

constructions! {
    construction item: Item {
        element ItemValue {}
        form item = "item";
    }
    abstract product Holder { items: seq Item separated by ", ", }
    require len(Holder.items) >= 2;
    require len(Holder.items) <= 4;
    abstract product HolderItemsSequenceCount3Category {}
}

fn main() {}
