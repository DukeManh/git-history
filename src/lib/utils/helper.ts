type classArgument = string | number | boolean | undefined | null;

export function classNames(...args: classArgument[]): string {
	return args.filter(Boolean).join(' ');
}
