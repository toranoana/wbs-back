use self::diesel::prelude::*;
use crate::db::manager::DataPgPool;
use crate::db::models::milestone::{Milestone, MilestoneNewForm, MilestoneUpdateForm};
use crate::graphql::schema::milestone::{NewMilestone, UpdateMilestone};
use crate::graphql::schema::Context;
use diesel::debug_query;
use diesel::pg::Pg;
use diesel::result::Error;
use log::debug;
use std::convert::TryInto;

extern crate diesel;

pub struct MilestoneRepository {}

/// TODO: 各メソッドの引数をcontextからDataPgPoolに統一したい
impl MilestoneRepository {
    // TODO: ここのエラーが不格好なので直したい(failure使うとか？)
    pub fn insert_milestone(
        context: &Context,
        new_milestone: NewMilestone,
    ) -> Result<Vec<Milestone>, Box<dyn std::error::Error>> {
        use crate::schema::milestones::dsl::*;
        use diesel::dsl::insert_into;
        let conn = &context.pool.get().unwrap();
        // PhotoFormのメンバは参照値なので参照値でintoかつライフタイムに注意
        let milestone_form: MilestoneNewForm = (&new_milestone).try_into()?;
        let rows_inserted = insert_into(milestones)
            .values(&milestone_form)
            .get_result(conn)
            .and_then(|m: Milestone| {
                milestones
                    .filter(project_id.eq(m.project_id))
                    .load::<Milestone>(conn)
            })?;
        Ok(rows_inserted)
    }

    pub fn update_milestone(
        context: &Context,
        pkey: i32,
        update_milestone: UpdateMilestone,
    ) -> Result<Milestone, Box<dyn std::error::Error>> {
        use crate::schema::milestones::dsl::*;
        use diesel::dsl::update;
        let conn = &context.pool.get().unwrap();
        let milestone_form: MilestoneUpdateForm = (&update_milestone).try_into()?;
        let rows_inserted = update(milestones.filter(id.eq(pkey)))
            .set(&milestone_form)
            .get_result(conn)?;
        Ok(rows_inserted)
    }

    pub fn any_project_milestones(
        pool: &DataPgPool,
        project_pkeys: &[i32],
    ) -> Result<Vec<Milestone>, Error> {
        use crate::schema::milestones::dsl::*;
        let conn = &pool.get().unwrap();
        let select_query = milestones.filter(project_id.eq_any(project_pkeys));
        let sql = debug_query::<Pg, _>(&select_query).to_string();
        debug!("{:?}", sql);
        select_query.load::<Milestone>(conn)
    }

    pub fn delete_milestone(
        context: &Context,
        milestone_pkey: i32,
    ) -> Result<Vec<Milestone>, Box<dyn std::error::Error>> {
        use crate::schema::milestones::dsl::*;
        use diesel::dsl::delete;
        let conn = &context.pool.get().unwrap();
        let project_pkey = milestones
            .find(milestone_pkey)
            .select(project_id)
            .get_result::<i32>(conn)?;

        let rows_deleted = delete(milestones.filter(id.eq(milestone_pkey)))
            .execute(conn)
            .and_then(|_| {
                milestones
                    .filter(project_id.eq(project_pkey))
                    .load::<Milestone>(conn)
            })?;
        Ok(rows_deleted)
    }
}
