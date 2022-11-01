import React from "react";

import { Box, Divider, Text, Flex, Button, Spacer } from '@chakra-ui/react'

// interface Data {
//     type: string,
//     man: string,
//     mpn: string
// }

export const PartsList = (data:any) => {
    return (
        <Box h='100px' borderWidth='1px' borderRadius='lg' mt={10}>
            <Flex>
                <Text ml='2'>Type {data.type}</Text>
                <Spacer />
                <Text>MAN.{data.man}</Text>
                <Spacer />
                <Text mr='2'>MPN {data.mpn}</Text>
            </Flex>
            <Divider />
            <Button colorScheme='teal' variant='outline' ml={2} mt={5}>Manual</Button>
        </Box>
    )
} 