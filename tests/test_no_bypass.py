from agentic_harness.plugin import Registry
from agentic_harness.runner import Runner


def test_registry_has_no_execute_helper():
    reg = Registry()
    assert not hasattr(reg, "execute")
    assert not hasattr(reg, "run_tool")
    assert hasattr(Runner, "run")
