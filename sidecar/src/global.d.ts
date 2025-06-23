declare module '*.txt' {
	const content: string;
	export default content;
}
declare global {
	namespace JSX {
		interface IntrinsicElements {
			[elemName: string]: Record<string, unknown> & { children?: React.ReactNode };
		}
	}
}
