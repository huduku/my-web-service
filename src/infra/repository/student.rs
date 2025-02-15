use crate::api::primitive::students::StuNo;
use crate::api::res::PageRes;
use crate::ddd::core::{DomainModel, Id, PageQuery};
use crate::app::cmd::student_cmd::StudentQuery;
use crate::domain::entity::student::Student;
use crate::domain::repo::student::StudentRepository;

use crate::ddd::repo::Repository;
use crate::infra::data::student::StudentDO;
use crate::infra::repository::DbRes;

use axum::extract::FromRef;
use rbatis::{Page, PageRequest};
use crate::context::CONTEXT;

pub struct StudentRepositoryImpl {}

impl Repository for StudentRepositoryImpl {
    
    type RawId = i64;
    type Id = Id<i64>;
    type Aggr = Student;

    async fn attach(&self, aggr: Student) {
        todo!()
    }

    async fn detach(&self, aggr: Student) {
        todo!()
    }

    async fn find(&self, id: Self::Id) -> Result<Self::Aggr, String> {
        let DbRes(res) = StudentDO::select_by_id(&CONTEXT.rb, id.0).await.into();
        match res {
            Ok(stu) => {
                match stu {
                    Some(student) => Student::new(&student),
                    None => Err("查询数据为空".to_string())
                }
            },
            Err(e) => Err(e)
        }
    }

    async fn save(&self, stu: Student) -> Result<(), String> {
        let res = match stu.id { 
            Some(_) => StudentDO::update_by_column(&CONTEXT.rb, &stu.into(), "id").await,
            None => StudentDO::insert(&CONTEXT.rb, &stu.into()).await,
        };
        
        match res {
            Ok(r) => {
                if r.rows_affected == 1 {
                    return Ok(())
                }
                Err("操作失败".to_string())
            },
            Err(e) => Err(e.to_string())
        }
    }

    async fn remove(&self, id: Id<i64>) -> Result<(), String> {
        let res = StudentDO::delete_by_column(&CONTEXT.rb, "id", id.0).await;
        match res {
            Ok(r) => {
                if r.rows_affected == 1 {
                    return Ok(())
                }
                Err("数据不存在".to_string())
            },
            Err(e) => Err(e.to_string())
        }
    }
}

impl StudentRepository for StudentRepositoryImpl {

    // type ID = Id<i64>;
    // type Aggr = Student;
    
    async fn find_one(&self, stu_no: StuNo) -> Result<Student, String> {
        let DbRes(res)  = 
            StudentDO::select_by_stu_no(&CONTEXT.rb, stu_no.0).await.into();
        match res {
            Ok(stu) => {
                match stu {
                    Some(student) => Student::new(&student),
                    None => Err("查询数据为空".to_string())
                }
            },
            Err(e) => Err(e)
        }
    }

    async fn find_page(&self, page_query: PageQuery<StudentQuery>) -> Result<PageRes<Student>, String> {
        let page: PageRequest = PageRequest::from_ref(&page_query);
        let stu = page_query.query.map(|q| q.into());
        let DbRes(db_res) =  
            DbRes::<Page<StudentDO>>::from(StudentDO::select_page(&CONTEXT.rb, &page, &stu).await);
        match db_res { 
            Ok(res) => Ok(PageRes::<Student>::try_from(res)?),
            Err(e) => Err(e)
        }
    }

    async fn count(&self, query: StudentQuery) -> Result<u64, String> {
        todo!()
    }
}