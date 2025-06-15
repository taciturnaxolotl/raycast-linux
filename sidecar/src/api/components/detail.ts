import { jsx } from 'react/jsx-runtime';
import { createWrapperComponent, createAccessorySlot } from '../utils';

const _AccessorySlot = createAccessorySlot();

const DetailPrimitive = createWrapperComponent('Detail');
const Detail = (props) => {
	const { metadata, actions, children, ...rest } = props;
	const metadataElement = metadata && jsx(_AccessorySlot, { name: 'metadata', children: metadata });
	const actionsElement = actions && jsx(_AccessorySlot, { name: 'actions', children: actions });
	return jsx(DetailPrimitive, {
		...rest,
		children: [children, metadataElement, actionsElement].filter(Boolean)
	});
};

const DetailMetadata = createWrapperComponent('Detail.Metadata');
const DetailMetadataLabel = createWrapperComponent('Detail.Metadata.Label');
const DetailMetadataLink = createWrapperComponent('Detail.Metadata.Link');
const DetailMetadataTagList = createWrapperComponent('Detail.Metadata.TagList');
const DetailMetadataTagListItem = createWrapperComponent('Detail.Metadata.TagList.Item');
const DetailMetadataSeparator = createWrapperComponent('Detail.Metadata.Separator');

Object.assign(Detail, {
	Metadata: DetailMetadata
});
Object.assign(DetailMetadata, {
	Label: DetailMetadataLabel,
	Link: DetailMetadataLink,
	TagList: DetailMetadataTagList,
	Separator: DetailMetadataSeparator
});
Object.assign(DetailMetadataTagList, {
	Item: DetailMetadataTagListItem
});

export { Detail };
