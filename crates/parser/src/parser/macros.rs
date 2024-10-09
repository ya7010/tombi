macro_rules! with_node {
    ($builder:expr, $kind:expr, $($content:tt)*) => {
        {
            $builder.start_node($kind.into());
            let res = $($content)*;
            $builder.finish_node();
            res
        }
    };
}
