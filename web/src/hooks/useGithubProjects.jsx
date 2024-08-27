import { useState, useEffect } from 'react';

function useGithubProjects(username) {
  const [projects, setProjects] = useState([]);
  const [error, setError] = useState(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    async function fetchProjects() {
      try {
        setLoading(true);
        const response = await fetch(`https://api.github.com/users/${username}/repos?type=owner`);
        const data = await response.json();
        setProjects(data);
        console.log(data);
        setLoading(false);
      } catch (error) {
        setError(error);
        setLoading(false);
      }
    }

    if (username) {
      fetchProjects();
    }
  }, [username]);

  return { projects, error, loading };

}

export default useGithubProjects;

// Path: src\hooks\useGithubProjects.jsx