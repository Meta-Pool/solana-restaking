# cargo install solana-verify
set -ex
PROGRAM_LIB_NAME=mp_sol_restaking
echo anchor-build to get target/idl and target/types 
anchor build -p $PROGRAM_LIB_NAME
echo solana-verify build to deploy
solana-verify build --library-name $PROGRAM_LIB_NAME
#solana-verify get-executable-hash target/deploy/$PROGRAM_LIB_NAME.so