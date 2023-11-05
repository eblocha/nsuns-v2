use tracing::{field::AsField, span, Subscriber, Value};
use tracing_subscriber::registry::LookupSpan;

pub trait FilterFn {
    fn enabled(&self, span: &span::Attributes<'_>) -> bool;
}

impl<F: Fn(&span::Attributes<'_>) -> bool> FilterFn for F {
    #[inline]
    fn enabled(&self, span: &span::Attributes<'_>) -> bool {
        self(span)
    }
}

pub struct AlwaysEnabled;

impl FilterFn for AlwaysEnabled {
    #[inline]
    fn enabled(&self, _span: &span::Attributes<'_>) -> bool {
        true
    }
}

pub struct GlobalFields<S, F: ?Sized + 'static, V, Filt, const N: usize> {
    inner: S,
    pairs: [(&'static F, V); N],
    filter: Filt,
}

impl<S, F: ?Sized, V, Filt, const N: usize> GlobalFields<S, F, V, Filt, N> {
    pub fn new(subscriber: S, pairs: [(&'static F, V); N], filter: Filt) -> Self {
        GlobalFields {
            inner: subscriber,
            pairs,
            filter,
        }
    }
}

impl<
        S: Subscriber,
        F: ?Sized + AsField + 'static,
        V: Value + 'static,
        Filt: FilterFn + 'static,
        const N: usize,
    > Subscriber for GlobalFields<S, F, V, Filt, N>
{
    #[inline]
    fn enabled(&self, metadata: &tracing::Metadata<'_>) -> bool {
        self.inner.enabled(metadata)
    }

    fn new_span(&self, span: &span::Attributes<'_>) -> span::Id {
        let id = self.inner.new_span(span);

        if self.filter.enabled(span) {
            let metadata = span.metadata();

            self.pairs
                .iter()
                .filter_map(|(field, value)| {
                    field
                        .as_field(metadata)
                        .map(|f| (f, Some(value as &dyn Value)))
                })
                .for_each(|(f, v)| {
                    // This isn't ideal but idk how to statically determine this.
                    // It should _technically_ be possible since span fields are set at compile time.
                    let pair = [(&f, v)];
                    // FIXME this is a hidden API
                    let values = span.fields().value_set(&pair);
                    let values = span::Record::new(&values);

                    self.record(&id, &values);
                });
        }
        id
    }

    #[inline]
    fn record(&self, span: &span::Id, values: &span::Record<'_>) {
        self.inner.record(span, values)
    }

    #[inline]
    fn record_follows_from(&self, span: &span::Id, follows: &span::Id) {
        self.inner.record_follows_from(span, follows)
    }

    #[inline]
    fn event(&self, event: &tracing::Event<'_>) {
        self.inner.event(event)
    }

    #[inline]
    fn enter(&self, span: &span::Id) {
        self.inner.enter(span)
    }

    #[inline]
    fn exit(&self, span: &span::Id) {
        self.inner.exit(span)
    }

    #[inline]
    fn on_register_dispatch(&self, subscriber: &tracing::Dispatch) {
        self.inner.on_register_dispatch(subscriber)
    }

    #[inline]
    fn register_callsite(
        &self,
        metadata: &'static tracing::Metadata<'static>,
    ) -> tracing::subscriber::Interest {
        self.inner.register_callsite(metadata)
    }

    #[inline]
    fn max_level_hint(&self) -> Option<tracing_subscriber::filter::LevelFilter> {
        self.inner.max_level_hint()
    }

    #[inline]
    fn event_enabled(&self, event: &tracing::Event<'_>) -> bool {
        self.inner.event_enabled(event)
    }

    #[inline]
    fn clone_span(&self, id: &span::Id) -> span::Id {
        self.inner.clone_span(id)
    }

    #[inline]
    fn drop_span(&self, id: span::Id) {
        #[allow(deprecated)]
        self.inner.drop_span(id)
    }

    #[inline]
    fn try_close(&self, id: span::Id) -> bool {
        self.inner.try_close(id)
    }

    #[inline]
    fn current_span(&self) -> tracing_core::span::Current {
        self.inner.current_span()
    }

    #[inline]
    unsafe fn downcast_raw(&self, id: std::any::TypeId) -> Option<*const ()> {
        self.inner.downcast_raw(id)
    }
}

impl<
        'span,
        S: LookupSpan<'span>,
        F: ?Sized + AsField + 'static,
        V: Value + 'static,
        Filt,
        const N: usize,
    > LookupSpan<'span> for GlobalFields<S, F, V, Filt, N>
{
    type Data = S::Data;

    #[inline]
    fn span_data(&'span self, id: &span::Id) -> Option<Self::Data> {
        self.inner.span_data(id)
    }
}

pub trait WithGlobalFields<F: ?Sized, V, const N: usize>
where
    Self: Sized,
{
    fn with_global_fields(
        self,
        pairs: [(&'static F, V); N],
    ) -> GlobalFields<Self, F, V, AlwaysEnabled, N> {
        self.with_global_fields_filtered(pairs, AlwaysEnabled)
    }
    fn with_global_fields_filtered<Filt>(
        self,
        pairs: [(&'static F, V); N],
        filter: Filt,
    ) -> GlobalFields<Self, F, V, Filt, N>;
}

impl<S, F: ?Sized, V, const N: usize> WithGlobalFields<F, V, N> for S {
    fn with_global_fields_filtered<Filt>(
        self,
        pairs: [(&'static F, V); N],
        filter: Filt,
    ) -> GlobalFields<Self, F, V, Filt, N> {
        GlobalFields::new(self, pairs, filter)
    }
}
