import { useState, useEffect } from 'react';
import axios from 'axios';

const GET_PINNED_REPOS = `
  query($username: String!) {
    user(login: $username) {
      pinnedItems(first: 6, types: REPOSITORY) {
        nodes {
          ... on Repository {
            id
            name
            description
            url
          }
        }
      }
    }
  }
`;

function usePinnedRepos(username) {
  const [pinnedRepos, setPinnedRepos] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    axios
      .post(
        'https://api.github.com/graphql',
        { query: GET_PINNED_REPOS, variables: { username } },
        { headers: { Authorization: `Bearer YOUR_GITHUB_ACCESS_TOKEN` } }
      )
      .then((response) => {
        setPinnedRepos(response.data.data.user.pinnedItems.nodes);
        setLoading(false);
      })
      .catch((error) => {
        console.error(error);
        setLoading(false);
      });
  }, [username]);

  return { pinnedRepos, loading };
}

export default usePinnedRepos;
