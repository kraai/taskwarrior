use failure::Fallible;
use taskchampion::Replica;
use termcolor::WriteColor;

pub(crate) fn execute<W: WriteColor>(w: &mut W, replica: &mut Replica) -> Fallible<()> {
    replica.gc()?;
    write!(w, "garbage collected.\n")?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::invocation::test::*;

    #[test]
    fn test_gc() {
        let mut w = test_writer();
        let mut replica = test_replica();
        execute(&mut w, &mut replica).unwrap();
        assert_eq!(&w.into_string(), "garbage collected.\n")
    }
}
