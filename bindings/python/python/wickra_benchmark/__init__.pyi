"""Type stubs for the wickra_benchmark package."""

__version__: str

class Benchmark:
    """A benchmark runner driven by JSON commands."""

    def __init__(self) -> None:
        """Create a benchmark runner.

        It is stateless — the case, suite and data arrive with each command.
        """
        ...

    def command(self, cmd_json: str) -> str:
        """Apply a command JSON and return the resulting response JSON.

        Raises ``ValueError`` if the command envelope cannot be parsed.
        """
        ...

    @staticmethod
    def version() -> str:
        """The library version."""
        ...
