from typing import Dict, Union


def get_dotenv_configuration(
    stage: str,
) -> Dict[str, Union[str, None]]:
    from dotenv import dotenv_values  # type: ignore

    return {
        **dotenv_values(f"../.env"),
        **dotenv_values(f"../.env.{stage}"),
    }
