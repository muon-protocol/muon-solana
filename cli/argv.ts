import yargs from 'yargs';

type MuonCommandHandlers = {
    initAdmin?: () => Promise<void>
    getAdminInfo?: () => Promise<void>
    transferAdmin?: (argv:any) => Promise<void>
    addGroup?: (argv:any) => Promise<void>
    listGroup?: () => Promise<void>
    verifyTest?: () => Promise<void>
    estimateLamports?: (argv:any) => Promise<void>
}

export async function handleArgs (handlers: MuonCommandHandlers) {
    await yargs(process.argv.slice(2))
        .command("admin-init",
            "initialize admin account.",
            {},
            handlers.initAdmin
        )
        .command("admin-get",
            "retrieve admin account info.",
            {},
            handlers.getAdminInfo
        )
        .command("admin-transfer <newAdmin>",
            "transfer admin to new account.",
            {
                newAdmin: {type: "string"},
            },
            handlers.transferAdmin
        )
        .command("group-add <ethAddress> <pubkeyX> <pubkeyYParity>",
            "add new verification group info.",
            {
                ethAddress: {type: "string"},
                pubkeyX: {type: "string"},
                pubkeyYParity: {type: "string"},
            },
            handlers.addGroup
        )
        .command("group-list",
            "list all groups already added.",
            {},
            handlers.listGroup
        )
        .command("verify-test",
            "call muon and verify its signature.",
            {},
            handlers.verifyTest
        )
        .demandCommand()
        .help()
        .argv;
}