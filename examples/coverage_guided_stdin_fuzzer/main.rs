use {
    king_david::{
        Fuzzer,
        corpora::InMemoryCorpus,
        deciders::{AlwaysResult, AndSplitDecider, WasTerminatedBySignal},
        executors::AflppStdinExecutor,
        feedbacks::{AlwaysInterestingFeedback, AndSplitFeedback, EdgeCoverageFeedback},
        loggers::StdoutDebugLogger,
        mutators::ByteMutator,
    },
    rand::{SeedableRng, rngs::SmallRng},
    std::env::args_os,
};

fn main() {
    let mut args = args_os();
    let _ = args.next(); // skip argv[0]
    let args: Vec<_> = args.collect();

    let mut rng = SmallRng::seed_from_u64(4 /* obtained by rolling a fair die */);

    let mut fuzzer = Fuzzer::new(
        AndSplitDecider::new(WasTerminatedBySignal::default(), AlwaysResult::default()),
        AflppStdinExecutor::new(&args[0], args.as_slice(), "/tmp/.cur_input"),
        AndSplitFeedback::new(
            AlwaysInterestingFeedback::default(),
            EdgeCoverageFeedback::default(),
        ),
        ByteMutator::new(SmallRng::from_rng(&mut rng)),
        InMemoryCorpus::new(
            rng,
            vec![vec![]], /* one element in the corpus: the empty string */
        ),
        StdoutDebugLogger::default(),
    );
    fuzzer.go();
}
