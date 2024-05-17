#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use Mk_AMEVTYPER1_EL0_Type::*;
use X_read::*;
use IsHighestEL::*;
use u__IMPDEF_boolean::*;
use IsG1ActivityMonitorImplemented::*;
use common::*;
pub fn AMEVTYPER1_EL0_SysRegWrite_622d8153064a03a6<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    op0: u8,
    op1: u8,
    CRn: u8,
    op2: u8,
    CRm: u8,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_80839: bool,
        el: u8,
        op0: u8,
        op1: u8,
        CRn: u8,
        op2: u8,
        CRm: u8,
        t: i128,
    }
    let fn_state = FunctionState {
        el,
        op0,
        op1,
        CRn,
        op2,
        CRm,
        t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #2s : i
        let s_0_0: i128 = 2;
        // S s_0_1: call IsG1ActivityMonitorImplemented(s_0_0)
        let s_0_1: bool = IsG1ActivityMonitorImplemented(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b7 b1
        if s_0_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: call IsHighestEL(s_1_1)
        let s_1_2: bool = IsHighestEL(state, tracer, s_1_1);
        // N s_1_3: branch s_1_2 b6 b2
        if s_1_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#80839 <= s_2_0
        fn_state.gs_80839 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#80839:u8
        let s_3_0: bool = fn_state.gs_80839;
        // N s_3_1: branch s_3_0 b5 b4
        if s_3_0 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: panic
        panic!("{:?}", ());
        // N s_4_1: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #64s : i64
        let s_5_0: i64 = 64;
        // D s_5_1: read-var t:i
        let s_5_1: i128 = fn_state.t;
        // D s_5_2: call X_read(s_5_1, s_5_0)
        let s_5_2: Bits = X_read(state, tracer, s_5_1, s_5_0);
        // D s_5_3: cast reint s_5_2 -> u64
        let s_5_3: u64 = (s_5_2.value() as u64);
        // D s_5_4: call Mk_AMEVTYPER1_EL0_Type(s_5_3)
        let s_5_4: ProductType5c790c8ef59cc8b2 = Mk_AMEVTYPER1_EL0_Type(
            state,
            tracer,
            s_5_3,
        );
        // C s_5_5: const #2s : i
        let s_5_5: i128 = 2;
        // C s_5_6: const #19152u : u32
        let s_5_6: u32 = 19152;
        // D s_5_7: read-reg s_5_6:[struct; 16]
        let s_5_7: [ProductType5c790c8ef59cc8b2; 16usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 16usize]>(s_5_6 as isize);
            tracer.read_register(s_5_6 as isize, value);
            value
        };
        // D s_5_8: mutate-element s_5_7[s_5_5] <= s_5_4
        let s_5_8: [ProductType5c790c8ef59cc8b2; 16usize] = {
            let mut local = s_5_7.clone();
            local[(s_5_5) as usize] = s_5_4;
            local
        };
        // D s_5_9: cast cvt s_5_8 -> [struct; 0]
        let s_5_9: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_5_8,
        );
        // D s_5_10: cast cvt s_5_9 -> [struct; 16]
        let s_5_10: [ProductType5c790c8ef59cc8b2; 16usize] = {
            let mut buf = [Default::default(); 16usize];
            buf.copy_from_slice(&s_5_9);
            buf
        };
        // C s_5_11: const #19152u : u32
        let s_5_11: u32 = 19152;
        // N s_5_12: write-reg s_5_11 <= s_5_10
        let s_5_12: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 16usize],
                >(s_5_11 as isize, s_5_10);
            tracer.write_register(s_5_11 as isize, s_5_10);
        };
        // N s_5_13: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #"AMEVCNTR1_EL0[m] is fixed" : str
        let s_6_0: &'static str = "AMEVCNTR1_EL0[m] is fixed";
        // S s_6_1: call __IMPDEF_boolean(s_6_0)
        let s_6_1: bool = u__IMPDEF_boolean(state, tracer, s_6_0);
        // S s_6_2: not s_6_1
        let s_6_2: bool = !s_6_1;
        // D s_6_3: write-var gs#80839 <= s_6_2
        fn_state.gs_80839 = s_6_2;
        // N s_6_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
}
