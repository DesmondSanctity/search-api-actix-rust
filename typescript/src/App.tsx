import { useState } from 'react';
import axios from 'axios';
import { PartsList } from './PartsList';
import { IconButton, Input, InputGroup, InputLeftAddon, Button, Flex, Text, Container } from '@chakra-ui/react'
import { SearchIcon } from '@chakra-ui/icons'


function App() {
  const [query, setQuery] = useState('')
  const [SearchResults, setSearchResults] = useState({})
  // ...
  const handleSubmit = (e: any) => {
    e.preventDefault()

    const uri = 'https://localhost/3000/query'
    axios.get(uri, { params: { q: query } })
      .then(response => {
        setSearchResults(response)
      })
      .catch(err => {
        console.log(err)
       })
  }

  const handleQueryChange = (e: any) => {
    setQuery(e.target.value)
  }

  return (
    <Container mt={10} >
      <Text>Add Via MPN</Text>
      <Flex gap={2}>
        <InputGroup>
          <InputLeftAddon children={<IconButton aria-label='Search database' icon={<SearchIcon />} />} />
          <Input type='text' placeholder='Input search query' onChange={handleQueryChange} />
        </InputGroup>
        <Button colorScheme='teal' variant='outline' onSubmit={handleSubmit}>Add Mock Data</Button>
      </Flex>
      <PartsList data={SearchResults} />
    </Container>
  );
}

export default App;

