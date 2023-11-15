import { Box, Button, Card, Heading } from "@chakra-ui/react";
import { rspc } from "../lib/ipc";
import { ApiObject, MangaAttributes } from '../lib/bindings';
import React from "react";
import MangaCmp from "../lib/componnents/MangaCmp";

export default function Home(){
    
    return (
        <React.Fragment>
            <Heading>Hello World</Heading>
            <PopularTitleList/>
        </React.Fragment>
    );
}


function PopularTitleList(){
    const {data, isSuccess, isFetching, refetch} = rspc.useQuery(["mdx-popular-titles"]);
    if(isSuccess){
        return (
            <React.Fragment>
                <Button isLoading={isFetching} onClick={() => {
                    refetch();
                }}>
                    Refetch
                </Button>
                <Box>
                {data.data.map((manga) => (
                    <MangaCmp value={manga} key={manga.id}/>
                ))}
            </Box>
            </React.Fragment>
            
        )
    }
    return (
        <React.Fragment>
            Loading...
        </React.Fragment>
    );
}
