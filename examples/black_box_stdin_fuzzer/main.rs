use {
    king_david::{
        Fuzzer, corpora::InMemoryCorpus, deciders::WasTerminatedBySignal,
        executors::BlackBoxStdinExecutor, feedbacks::AlwaysInterestingFeedback,
        loggers::StdoutDebugLogger, mutators::ByteMutator,
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
        WasTerminatedBySignal::default(),
        BlackBoxStdinExecutor::new(args[0].clone(), args),
        AlwaysInterestingFeedback::default(),
        ByteMutator::new(SmallRng::from_rng(&mut rng)),
        InMemoryCorpus::new(
            rng,
            vec![vec![]], /* one element in the corpus: the empty string */
        ),
        StdoutDebugLogger::default(),
    );
    fuzzer.go();
}
