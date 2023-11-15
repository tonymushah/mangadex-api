import { Card, CardBody, Heading, Text } from "@chakra-ui/react"
import { ApiObject, MangaAttributes } from '../bindings';

export default function MangaCmp({ value } : {
    value: ApiObject<MangaAttributes>
}){
    return (
        <Card>
            <CardBody>
                <Heading>{value.attributes.title.en ?? "NotFound"}</Heading>
                <Text>{value.attributes.description.en ?? "NotFound"}</Text>
            </CardBody>
        </Card>
    );
}
