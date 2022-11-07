import React from "react";

import { Box, Divider, Text, Flex, Button, Spacer } from '@chakra-ui/react'

export const PartsList = (data: any, highlight: string) => {
    console.log(data, highlight)

    // Here will be a useEffect hook that takes the highlight string
    //  and matches it with the DOM elements with classnames "highlight"
    
    return (
        <>
            {/* Here should be where we map through data from part_list to
         show the parts from the API response */}
            <Box h='100px' borderWidth='1px' borderRadius='lg' mt={10}>
                <Flex mt={3}>
                    <Text as='b' mr='2' ml={2}>
                        TYPE
                    </Text>
                    <Text as='b' color='gray' >
                        Resistors
                    </Text>
                    <Spacer />
                    <Text as='b' mr={2} ml={2}>
                        MAN.
                    </Text>
                    <Text as='b' color='gray'>
                        Delta
                    </Text>
                    <Spacer />
                    <Text as='b' mr='2' ml={2}>
                        MPN
                    </Text>
                    <Text as='b' mr={2} color='gray' className="highlight">
                        BCDEF
                    </Text>
                </Flex>
                <Divider mt={3} />
                <Button colorScheme='teal' variant='outline' ml={2} mt={2} size={"sm"}>Manual</Button>
            </Box>

            <Box h='100px' borderWidth='1px' borderRadius='lg' mt={10}>
                <Flex mt={3}>
                    <Text as='b' mr='2' ml={2}>
                        TYPE
                    </Text>
                    <Text as='b' color='gray' >
                        Resistors
                    </Text>
                    <Spacer />
                    <Text as='b' mr={2} ml={2}>
                        MAN.
                    </Text>
                    <Text as='b' color='gray'>
                        Delta
                    </Text>
                    <Spacer />
                    <Text as='b' mr='2' ml={2}>
                        MPN
                    </Text>
                    <Text as='b' mr={2} color='gray'>
                        BC-DEGH
                    </Text>
                </Flex>
                <Divider mt={3} />
                <Button colorScheme='teal' variant='outline' ml={2} mt={2} size={"sm"}>Manual</Button>
            </Box>
        </>
    )
} 