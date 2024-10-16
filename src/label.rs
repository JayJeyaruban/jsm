#[derive(Debug, PartialEq, Eq)]
pub struct Labelled<L, I: ?Sized> {
    label: L,
    item: I,
}

pub trait WithLabel {
    fn with_label<L>(self, label: L) -> Labelled<L, Self>;
}

impl<T> WithLabel for T {
    fn with_label<L>(self, label: L) -> Labelled<L, Self> {
        Labelled { label, item: self }
    }
}

impl<L, I> Labelled<L, I> {
    pub fn label(&self) -> &L {
        &self.label
    }

    pub fn map<N>(self, fun: impl Fn(I) -> N) -> Labelled<L, N> {
        let Labelled { label, item } = self;
        let next = fun(item);

        Labelled { label, item: next }
    }
}

impl<L, I> AsRef<I> for Labelled<L, I> {
    fn as_ref(&self) -> &I {
        &self.item
    }
}

pub trait LabelledVec<Label, SearchLabel, T> {
    fn find_by_label(&self, label: SearchLabel) -> Option<&Labelled<Label, T>>;
}

impl<Label, SearchLabel, T> LabelledVec<Label, SearchLabel, T> for Vec<Labelled<Label, T>>
where
    Label: PartialEq<SearchLabel>,
    SearchLabel: PartialEq<Label>,
{
    fn find_by_label(&self, label: SearchLabel) -> Option<&Labelled<Label, T>> {
        self.iter().find(|item| item.label == label)
    }
}

#[test]
fn test_label() {
    struct Item(&'static str);

    let item = Item(" world");
    let item = item.with_label("Hello");

    assert_eq!(
        "Hello world",
        format!("{}{}", item.label(), item.as_ref().0)
    );
}

#[test]
fn find_by_label_test() {
    let things = vec![
        1.with_label("Hello".to_string()),
        2.with_label("World".to_string()),
    ];

    assert_eq!(
        Some(2),
        things.find_by_label("World").map(|res| *(res.as_ref()))
    );
}

#[test]
fn label_map_test() {
    let item = 3.with_label("label");
    let item = item.map(|_| 234);

    assert_eq!(item, 234.with_label("label"));
}
