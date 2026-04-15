**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

# Delta Wrecker

**Delta Wrecker** is an orbital salvage game utilizing real mechanics to rescue satellites and clear space debris. Experience the challenge of running a one-ship salvage company: performing precision burns, intercepts, grappling, and towing targets to graveyard orbits or repair stations.

## Prerequisites

Before you begin, ensure you have the following installed:
- **Rust** (latest stable)
- **Python** (3.8+)
- **uv** (for Python project and environment management)

## Setup and Build Instructions

We use `uv` for managing the Python environment and dependencies.

1. **Clone the repository:**
   ```bash
   git clone https://github.com/fred-machuga/delta-wrecker.git
   cd delta-wrecker/game
   ```

2. **Activate your virtual environment (if you have one):**
   If you already have a `.venv` set up, be sure to activate it (e.g., `.\.venv\Scripts\activate` on Windows, or `source .venv/bin/activate` on macOS/Linux).

3. **Install dependencies and setup the virtual environment:**
   ```bash
   uv sync
   ```

4. **Build the Rust extension using maturin:**
   ```bash
   uv run maturin develop --manifest-path ../engine/Cargo.toml
   ```
   *(Note: This compiles the Rust core logic and correctly links it into the Python virtual environment).*

5. **Run the application:**
   ```bash
   uv run main.py
   ```

### Alternative Setup (Without `uv`)

If you do not have `uv` installed or experience issues, you can manually set up and run the environment using standard Python tools:

```bash
# Make sure you are in the game directory
cd delta-wrecker/game

# Create and activate the virtual environment
python -m venv .venv
# On Windows:
.\.venv\Scripts\activate
# On macOS/Linux:
source .venv/bin/activate

# Install dependencies manually
python -m pip install pygame maturin pytest

# Build the Rust extension
maturin develop --manifest-path ../engine/Cargo.toml

# Run the application
python main.py
```

**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.