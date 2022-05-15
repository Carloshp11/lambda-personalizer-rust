alias ll='ls -lh'
alias kk='bash kustomizer.sh'
# tb() {
#     if [[ $1 != "--token" ]]; then
#         command tb --token "$TB_TOKEN" "$@"
#     else
#         command tb "$@"
#     fi
# }

if [ -z "TB_HOST" ]; then echo "TB_HOST not defined"; fi
if [ -z "TB_TOKEN" ]; then echo "TB_TOKEN not defined"; fi
# echo "TB_TOKEN= $TB_TOKEN"
# tb auth --host "$TB_HOST"
# tb auth --host "$TB_HOST" <<< $"$TB_TOKEN" || exit 1