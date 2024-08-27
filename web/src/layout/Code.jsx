import { useMemo, useEffect } from 'react';
import useGithubProjects from '@/hooks/useGithubProjects';
import styles from '@/styles/Code.module.scss';

function Code({ username }) {
  const { projects, loading, error } = useGithubProjects(username);

  const memoizedProjects = useMemo(() => {
    return projects;
  }, [projects]);

  if (error) {
    return <div className={styles.error}>Error: {error.message}</div>;
  }

  if (loading) {
    return <div className={styles.loading}>Loading...</div>;
  }

  return (
    <div className={styles.projectList}>
      <h2>My Projects on GitHub</h2>
      <div className={styles.projectContainer}>
        {memoizedProjects.map(project => (
          <div key={project.id} className={styles.project}>
            <h3>{project.name}</h3>
            <p>{project.description}</p>
            <div className={styles.projectLinks}>
              <a href={project.html_url} target="_blank" rel="noopener noreferrer">GitHub Repository</a>
              {project.homepage && <a href={project.homepage} target="_blank" rel="noopener noreferrer">Live Demo</a>}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default Code;
