use std::marker::PhantomData;

struct Nothing {}

/// Lazy

trait Lazy<A> {
    type Input;
    type Output;
    type Kind: HKT;
    type Lifted;

    fn kind() -> Self::Kind;
    fn run(&self, i: Self::Input) -> A;
}

struct Void {}
struct LazyRoot<K: HKT, A> {__marker: PhantomData<*const A>, __marker_k: K }

struct LazyMap<F, I, A> {
    op_input: PhantomData<*const A>,
    op: F,
    inner: I
}

impl<F,I, A,B> Lazy<B> for LazyMap<F,I, A> where F: Fn(A) -> B, B: Lazy<B>, I: Lazy<A,Kind=OptionKind> {
    type Input = I::Input;
    type Output = B;
    type Kind = OptionKind;
    type Lifted = Option<B>;

    fn kind() -> <Self as Lazy<A>>::Kind {
        unimplemented!()
    }


    fn run(&self, i: <Self as Lazy<B>>::Input) -> <Self as Lazy<B>>::Output {
        (self.op)(self.inner.run(i))
    }
}
impl<F: HKT, A: Lift> Lazy<A> for LazyRoot<F, A> {
    type Input = A;
    type Output = A;
    type Kind = F;
    type Lifted = A::Lifted;

    fn kind() -> <Self as Lazy<A>>::Kind {
        Self::Kind::repr()
    }


    fn run(&self, i: <Self as Lazy<A>>::Input) -> <Self as Lazy<A>>::Output {
        i
    }
}
struct Kind<K: HKT, A> {
    __marker_k: K,
    __marker_a: PhantomData<*const A>
}


struct Lifted<A: Lazy<A>> {
    kind: A::Kind,
    inner: A
}

impl<A> Lazy<A> for Lifted<A> where A: Lazy<A> {
    type Input = A::Input;
    type Output = A::Output;
    type Kind = A::Kind;
    type Lifted = A::Lifted;


    fn kind() -> <Self as Lazy<A>>::Kind {
        Self::Kind::repr()
    }

    fn run(&self, i: <Self as Lazy<A>>::Input) -> <Self as Lazy<A>>::Output {
        self.inner.run(i)
    }
}

trait LazyMapExt<A: Lazy<A>> {
    type Out;
    fn map<F,B>(self, f: F) -> LazyMap<F, Self::Out, B> where Self: Sized, B: Lazy<B>, F: Fn(A::Lifted) -> B::Lifted;
}

struct LazyOption<A, I>(A, I);

impl<A, I: Lazy<Option<A>>> Lazy<A> for LazyOption<A, I> {
    type Input = I::Input;
    type Output = Option<A>;
    type Kind = OptionKind;
    type Lifted = Option<A>;

    fn kind() -> <Self as Lazy<A>>::Kind {
        OptionKind
    }

    fn run(&self, i: <Self as Lazy<A>>::Input) -> <Self as Lazy<A>>::Output {
        self.1.run(i)
    }
}



impl<A> Lift for Option<A> {
    type Kind = OptionKind;
    type Unlifted = A;
    type Lifted = Option<A>;

    fn lift(&self) -> Lifted<LazyRoot<Self::Kind, Self>> where Self: Sized {
        Lifted {
            kind: OptionKind,
            inner: LazyRoot{__marker: PhantomData, __marker_k: OptionKind}
        }
    }
}

trait Functor<F_: HKT> {
    fn map<F,A,B>(fa: Lifted<A>, f: F) -> Lifted<B> where
        F: Fn(A) -> B,
        A: Lazy<A,Kind=F_>,
        B: Lazy<B,Kind=F_>;
}

impl Functor<OptionKind> for OptionKind {
    fn map<F, A, B>(fa: Lifted<A>, f: F) -> Lifted<B> where
        F: Fn(A) -> B,
        A: Lazy<A,Kind=OptionKind>,
        B: Lazy<B, Kind=OptionKind> {
        fa.map(|o| o.map(f))
    }
}
#[test]
fn test_lift() {
    let opt = Some(1);
    let lifted = opt.lift().map(|i| i.map(|i| i * 2));
    opt.lift();
    lifted.run(opt);
}
/// Kind


trait HKT: 'static {
    type Kind: HKT;
    fn repr() -> Self;
}

#[derive(Debug)]
struct Empty;
impl HKT for Empty {
    type Kind = Empty;

    fn repr() -> Self {
        Empty
    }
}

#[derive(Debug)]
struct IdKind;
impl HKT for IdKind {
    type Kind = IdKind;

    fn repr() -> Self {
        IdKind
    }
}

#[derive(Debug)]
struct OptionKind;
impl HKT for OptionKind {

    type Kind = OptionKind;

    fn repr() -> Self {
        OptionKind
    }
}



