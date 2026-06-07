# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_umbrella_fetch_global_optspecs
	string join \n watch= h/help
end

function __fish_umbrella_fetch_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_umbrella_fetch_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_umbrella_fetch_using_subcommand
	set -l cmd (__fish_umbrella_fetch_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c umbrella-fetch -n "__fish_umbrella_fetch_needs_command" -l watch -d 'Watch feed continuously (seconds)' -r
complete -c umbrella-fetch -n "__fish_umbrella_fetch_needs_command" -s h -l help -d 'Print help'
complete -c umbrella-fetch -n "__fish_umbrella_fetch_needs_command" -f -a "full" -d 'Full secure feed'
complete -c umbrella-fetch -n "__fish_umbrella_fetch_needs_command" -f -a "ubcs" -d 'U.B.C.S. roster and status'
complete -c umbrella-fetch -n "__fish_umbrella_fetch_needs_command" -f -a "virus" -d 'Virus strain simulation'
complete -c umbrella-fetch -n "__fish_umbrella_fetch_needs_command" -f -a "uss" -d 'U.S.S. classified roster'
complete -c umbrella-fetch -n "__fish_umbrella_fetch_needs_command" -f -a "minimal" -d 'Minimal system fetch'
complete -c umbrella-fetch -n "__fish_umbrella_fetch_needs_command" -f -a "fetch" -d 'Alias for minimal fetch'
complete -c umbrella-fetch -n "__fish_umbrella_fetch_needs_command" -f -a "completions" -d 'Generate shell completions'
complete -c umbrella-fetch -n "__fish_umbrella_fetch_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand full" -l watch -d 'Watch feed continuously (seconds)' -r
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand full" -s h -l help -d 'Print help'
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand ubcs" -l squad -d 'Filter by squad' -r
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand ubcs" -l watch -d 'Watch feed continuously (seconds)' -r
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand ubcs" -l list -d 'List roster'
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand ubcs" -s h -l help -d 'Print help'
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand virus" -l strain -d 'Select strain' -r
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand virus" -l watch -d 'Watch feed continuously (seconds)' -r
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand virus" -l list -d 'List strains'
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand virus" -s h -l help -d 'Print help'
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand uss" -l squad -d 'Filter by squad' -r
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand uss" -l status -d 'Filter by status' -r
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand uss" -l watch -d 'Watch feed continuously (seconds)' -r
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand uss" -l list -d 'List classified roster'
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand uss" -s h -l help -d 'Print help'
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand minimal" -l watch -d 'Watch feed continuously (seconds)' -r
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand minimal" -s h -l help -d 'Print help'
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand fetch" -l watch -d 'Watch feed continuously (seconds)' -r
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand fetch" -s h -l help -d 'Print help'
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand completions" -l watch -d 'Watch feed continuously (seconds)' -r
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand completions" -s h -l help -d 'Print help'
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand help; and not __fish_seen_subcommand_from full ubcs virus uss minimal fetch completions help" -f -a "full" -d 'Full secure feed'
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand help; and not __fish_seen_subcommand_from full ubcs virus uss minimal fetch completions help" -f -a "ubcs" -d 'U.B.C.S. roster and status'
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand help; and not __fish_seen_subcommand_from full ubcs virus uss minimal fetch completions help" -f -a "virus" -d 'Virus strain simulation'
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand help; and not __fish_seen_subcommand_from full ubcs virus uss minimal fetch completions help" -f -a "uss" -d 'U.S.S. classified roster'
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand help; and not __fish_seen_subcommand_from full ubcs virus uss minimal fetch completions help" -f -a "minimal" -d 'Minimal system fetch'
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand help; and not __fish_seen_subcommand_from full ubcs virus uss minimal fetch completions help" -f -a "fetch" -d 'Alias for minimal fetch'
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand help; and not __fish_seen_subcommand_from full ubcs virus uss minimal fetch completions help" -f -a "completions" -d 'Generate shell completions'
complete -c umbrella-fetch -n "__fish_umbrella_fetch_using_subcommand help; and not __fish_seen_subcommand_from full ubcs virus uss minimal fetch completions help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
