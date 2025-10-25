#!/bin/bash

# 1. builds project
# 2. sets up git
# 3. pushes to gitpages




logok() {
    echo -e "\033[32m$@\033[0m"
}
logerr() {
    echo -e "\033[31m$@\033[0m"
}
run() {
  "$@"
  if [ $? -ne 0 ]; then
    logerr "Command failed: $@"
    exit 1
  fi
}



THIS_COMMIT_NAME=$(git log --format=%B -n 1 $(git rev-parse --abbrev-ref HEAD))
THIS_COMMIT_HASH=$(git rev-parse HEAD)
THIS_GIT_COMMIT_NAME="$THIS_COMMIT_NAME-($THIS_COMMIT_HASH)"
logok "Current git branch: $THIS_GIT_COMMIT_NAME"
if [ "$(git rev-parse --abbrev-ref HEAD)" != "main" ]; then
    logerr "You are not on main branch"
    exit 1
fi





# BUILDS
run sh proj_build_release.sh
logok "Built project"



# SETUP GIT
# oh it`s ok actually





# PUSH TO GITPAGES
GITPAGES_FOLDER="D:/PR/yangvar6.github.io"
FILES_TO_IGNORE=( "CNAME" ".gitignore"  "LICENSE"  "README.md" )

# delete all files in gitpages folder
for file in $(ls "$GITPAGES_FOLDER"); do
    if [[ ! " ${FILES_TO_IGNORE[@]} " =~ " ${file} " ]]; then
        run rm "$GITPAGES_FOLDER/$file"
        logok "Deleted $file"
    fi
done
logok "Deleted all files in gitpages folder"

# copy files to gitpages folder
for file in $(ls ./dist); do
    run cp "./dist/$file" "$GITPAGES_FOLDER/$file"
done
logok "Copied files to gitpages folder"


NEW_GIT_COMMIT_NAME="release-of $THIS_GIT_COMMIT_NAME"
logok "New git commit name: $NEW_GIT_COMMIT_NAME"
pushd "$GITPAGES_FOLDER"
    run git add .
    git remote add origin https://github.com/yangvar6/yangvar6.website.git
    run git commit -m "$NEW_GIT_COMMIT_NAME"
    git push -u origin main
popd






logok "Push completed!!!"