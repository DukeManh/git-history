type classArgument = string | number | boolean | undefined | null;

export function classNames(...args: classArgument[]): string {
	return args.filter(Boolean).join(' ');
}

export function splitLines(str: string): string[] {
	return str.split(/\r?\n/g);
}
