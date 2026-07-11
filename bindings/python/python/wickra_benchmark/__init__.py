"""Wickra Benchmark — recompute a curated case or suite and confirm it.

Create a :class:`Benchmark`, drive it with command JSONs (``run_case``,
``run_suite``, ``list_cases``, ``version``) and read back response JSONs. The
same command protocol crosses every language binding, so this Python front-end
drives the exact same core as the native CLI.
"""

from ._wickra_benchmark import Benchmark, __version__

__all__ = ["Benchmark", "__version__"]
