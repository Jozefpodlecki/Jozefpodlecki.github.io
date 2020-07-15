import { Project, Page, ProjectSearchCriteria, Tag } from "@models";
import projectsJson from 'data/projects.json';
import tagsJson from 'data/tags.json';

const projectsTags = projectsJson.filter(project => project.active).reduce((acc, project) => [...acc, ...project.skills], []) as string[];

const defaultCriteria: ProjectSearchCriteria = {
  active: true,
  pageSize: 10,
  page: 0,
  tags: [],
  name: ''
}

export const getProjects = (criteria: ProjectSearchCriteria) => new Promise<Page<Project>>((resolve, reject) => {
    let projects = <Project[]>projectsJson;

    criteria = {
      ...defaultCriteria,
      ...criteria
    }

    projects = projects.map((project, index) => ({
        ...project,
        thumbnail: project.thumbnail || 'assets/projects/thumbnail.png',
        preview: project.preview || 'assets/projects/thumbnail.webm',
        rank: project.rank || 1
    }))
    
    projects.sort((prev, next) => {
      return prev.rank > next.rank ? 1 : -1
    })

    let {
      active,
      pageSize,
      page = null,
      tags = null,
      name = null } = criteria

    if(tags && tags.length) {
        projects = projects.filter(project => {
            return project.skills.some(skill => criteria.tags.includes(skill))
        })
        
    }
    
    projects = projects.filter(project => project.active === active);
    
    if(name) {
      name = name.toLowerCase();
      projects = projects.filter(project => project.name.toLowerCase().includes(name));
     
    }

    const total = Math.ceil(projects.length / pageSize);

    if(page !== null) {
      const items = page * pageSize;
      projects = projects.slice(items, items + pageSize)
    }

    const result = {
      data: projects,
      total
    }
    
    resolve(result);
})

export const getTags = () => new Promise<Tag[]>((resolve, reject) => {
  const tags = tagsJson as Tag[];

  const computed = tags.filter(tag => projectsTags.includes(tag.name))

  resolve(computed);
})