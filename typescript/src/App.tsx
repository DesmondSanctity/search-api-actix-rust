import React from 'react';
import { PartsList } from './PartsList';
import { IconButton, Input, InputGroup, InputLeftAddon, Button, Flex, Text, Container } from '@chakra-ui/react'
import { SearchIcon } from '@chakra-ui/icons'



function App() {
  return (
    <Container mt={10} >
        <Text>Add Via MPN</Text>
      <Flex gap={2}>
        <InputGroup>
          <InputLeftAddon children={<IconButton aria-label='Search database' icon={<SearchIcon />} />} />
          <Input type='text' placeholder='Input search query' />
        </InputGroup>
        <Button colorScheme='teal' variant='outline'>Add Mock Data</Button>
      </Flex>
      <PartsList type={''} man={''} mpn={''} />
    </Container>
  );
}

export default App;
