import os
from typing import Dict, Union


def get_config(stage: str) -> Dict[str, Union[str, None]]:
    from dotenv import dotenv_values  # type: ignore

    return {
        **dotenv_values(f"../.env"),
        **dotenv_values(f"../.env.{stage}"),
        **os.environ,  # override loaded values with environment variables
    }


def get_env_var(name: str, stage: str) -> str:
    config = get_config(stage)

    if name not in config or config[name] is None:
        raise KeyError(f"Missing environment variable {name}")
    return config[name]  # type: ignore
