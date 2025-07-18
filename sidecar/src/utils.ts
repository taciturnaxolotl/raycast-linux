import React from 'react';
import type { ComponentType, ParentInstance } from './types';
import { root, instances } from './state';
import type { Command } from '@raycast-linux/protocol';

export const getComponentDisplayName = (type: ComponentType): string => {
	if (typeof type === 'string') {
		return type;
	}
	return type.displayName ?? type.name ?? 'Anonymous';
};

export function serializeProps(props: Record<string, unknown>): Record<string, unknown> {
	const serialized: Record<string, unknown> = {};

	for (const key in props) {
		// part 1: we don't need to serialize children because they are handled by the reconciler
		if (key === 'children') {
			continue;
		}

		const value = props[key];

		if (typeof value === 'function') {
			serialized[key] = true;
			continue;
		}

		// part 2: deep-serialize react elements if they appear in props
		if (React.isValidElement(value)) {
			serialized[key] = {
				$$typeof: 'react.element.serialized',
				type: getComponentDisplayName(value.type as ComponentType),
				props: serializeProps(value.props as Record<string, unknown>)
			};
			continue;
		}

		// part 3: recursively serialize arrays because they might contain elements
		if (Array.isArray(value)) {
			serialized[key] = value.map((item) =>
				React.isValidElement(item)
					? {
							$$typeof: 'react.element.serialized',
							type: getComponentDisplayName(item.type as ComponentType),
							props: serializeProps(item.props as Record<string, unknown>)
						}
					: item
			);
			continue;
		}

		// part 4: we don't need to serialize the value, just copy it directly
		serialized[key] = value;
	}

	return serialized;
}

export function optimizeCommitBuffer(buffer: Command[]): Command[] {
	const OPTIMIZATION_THRESHOLD = 10;
	const childOpsByParent = new Map<ParentInstance['id'], Command[]>();
	const otherOps: Command[] = [];

	for (const op of buffer) {
		const isChildOp =
			op.type === 'APPEND_CHILD' || op.type === 'REMOVE_CHILD' || op.type === 'INSERT_BEFORE';

		if (isChildOp) {
			const parentId = op.payload.parentId;
			childOpsByParent.set(parentId, (childOpsByParent.get(parentId) ?? []).concat(op));
		} else {
			otherOps.push(op);
		}
	}

	if (childOpsByParent.size === 0) {
		return buffer;
	}

	const finalOps = [...otherOps];

	for (const [parentId, ops] of childOpsByParent.entries()) {
		if (ops.length <= OPTIMIZATION_THRESHOLD) {
			finalOps.push(...ops);
			continue;
		}

		const parentInstance = parentId === 'root' ? root : instances.get(parentId as number);

		if (parentInstance && 'children' in parentInstance) {
			const childrenIds = parentInstance.children.map(({ id }) => id);
			finalOps.push({
				type: 'REPLACE_CHILDREN',
				payload: { parentId, childrenIds }
			});
		} else {
			finalOps.push(...ops);
		}
	}

	return finalOps;
}
