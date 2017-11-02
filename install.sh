#!/bin/bash -ex
# Install rustfmt
if which rustfmt > /dev/null; then
  echo "* rustfmt is already installed"
else
  cargo install rustfmt
fi

# Install commit hooks
if [ -d '.git/hooks' ] && [ ! -f '.git/hooks/pre-commit' ] ; then
  ln -s ../../git-hooks/pre-commit .git/hooks
  echo 'git pre-commit hook is installed'
else
  echo '* git pre-commit hook is already installed'
fi
