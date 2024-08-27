#!/bin/bash

# Detect the OS
OS=$(uname)

if [[ "$OS" == "Linux" ]]; then
  # Determine Linux distribution
  if [ -f /etc/os-release ]; then
    . /etc/os-release
    DISTRO=$ID
  fi

  # Update package lists and install FFmpeg
  if [[ "$DISTRO" == "ubuntu" ]]; then
    echo "Updating package lists"
    sudo apt-get update

    echo "Installing FFmpeg"
    sudo apt-get install -y ffmpeg
  elif [[ "$DISTRO" == "fedora" ]]; then
    echo "Updating package lists"
    sudo dnf check-update

    echo "Installing FFmpeg"
    sudo dnf install -y ffmpeg
  fi

elif [[ "$OS" == "Darwin" ]]; then  # MacOS
  echo "Installing FFmpeg"
  brew install ffmpeg
fi

# Check if the virtual environment directory exists; if not, create it
if [ ! -d "venv" ]; then
  echo "Creating virtual environment"
  python3 -m venv venv
fi

# Activate the virtual environment
echo "Activating virtual environment"
source venv/bin/activate

# Upgrade pip
echo "Upgrading pip"
pip install --upgrade pip

# Install packages from requirements.txt
echo "Installing packages from requirements.txt"
pip install -r requirements.txt

echo "Activating virtual environment"
source venv/bin/activate