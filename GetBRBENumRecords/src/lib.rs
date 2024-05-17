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
use u_get_BRBIDR0_EL1_Type_NUMREC::*;
use u__IMPDEF_integer::*;
use common::*;
pub fn GetBRBENumRecords<T: Tracer>(state: &mut State, tracer: &T, gs_3404: ()) -> i128 {
    #[derive(Default)]
    struct FunctionState {
        gs_3406: bool,
        gs_3407: bool,
        gs_3405: bool,
        gs_3404: (),
    }
    let fn_state = FunctionState {
        gs_3404,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_0_0: const #13496u : u32
        let s_0_0: u32 = 13496;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call _get_BRBIDR0_EL1_Type_NUMREC(s_0_1)
        let s_0_2: u8 = u_get_BRBIDR0_EL1_Type_NUMREC(state, tracer, s_0_1);
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 8u16);
        // D s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (s_0_3.value() as i128);
        // D s_0_5: cast reint s_0_4 -> i64
        let s_0_5: i64 = (s_0_4 as i64);
        // C s_0_6: const #8u : u8
        let s_0_6: u8 = 8;
        // C s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 8u16);
        // C s_0_8: cast zx s_0_7 -> i
        let s_0_8: i128 = (s_0_7.value() as i128);
        // C s_0_9: cast reint s_0_8 -> i64
        let s_0_9: i64 = (s_0_8 as i64);
        // D s_0_10: cast zx s_0_5 -> i
        let s_0_10: i128 = (i128::try_from(s_0_5).unwrap());
        // C s_0_11: cast zx s_0_9 -> i
        let s_0_11: i128 = (i128::try_from(s_0_9).unwrap());
        // D s_0_12: cmp-eq s_0_10 s_0_11
        let s_0_12: bool = ((s_0_10) == (s_0_11));
        // N s_0_13: branch s_0_12 b9 b1
        if s_0_12 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_1_0: const #13496u : u32
        let s_1_0: u32 = 13496;
        // D s_1_1: read-reg s_1_0:struct
        let s_1_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: call _get_BRBIDR0_EL1_Type_NUMREC(s_1_1)
        let s_1_2: u8 = u_get_BRBIDR0_EL1_Type_NUMREC(state, tracer, s_1_1);
        // D s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 8u16);
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (s_1_3.value() as i128);
        // D s_1_5: cast reint s_1_4 -> i64
        let s_1_5: i64 = (s_1_4 as i64);
        // C s_1_6: const #16u : u8
        let s_1_6: u8 = 16;
        // C s_1_7: cast zx s_1_6 -> bv
        let s_1_7: Bits = Bits::new(s_1_6 as u128, 8u16);
        // C s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (s_1_7.value() as i128);
        // C s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // D s_1_10: cast zx s_1_5 -> i
        let s_1_10: i128 = (i128::try_from(s_1_5).unwrap());
        // C s_1_11: cast zx s_1_9 -> i
        let s_1_11: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_12: cmp-eq s_1_10 s_1_11
        let s_1_12: bool = ((s_1_10) == (s_1_11));
        // N s_1_13: branch s_1_12 b8 b2
        if s_1_12 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_2_0: const #13496u : u32
        let s_2_0: u32 = 13496;
        // D s_2_1: read-reg s_2_0:struct
        let s_2_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: call _get_BRBIDR0_EL1_Type_NUMREC(s_2_1)
        let s_2_2: u8 = u_get_BRBIDR0_EL1_Type_NUMREC(state, tracer, s_2_1);
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 8u16);
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (s_2_3.value() as i128);
        // D s_2_5: cast reint s_2_4 -> i64
        let s_2_5: i64 = (s_2_4 as i64);
        // C s_2_6: const #32u : u8
        let s_2_6: u8 = 32;
        // C s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 8u16);
        // C s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (s_2_7.value() as i128);
        // C s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: cast zx s_2_5 -> i
        let s_2_10: i128 = (i128::try_from(s_2_5).unwrap());
        // C s_2_11: cast zx s_2_9 -> i
        let s_2_11: i128 = (i128::try_from(s_2_9).unwrap());
        // D s_2_12: cmp-eq s_2_10 s_2_11
        let s_2_12: bool = ((s_2_10) == (s_2_11));
        // N s_2_13: branch s_2_12 b7 b3
        if s_2_12 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_3_0: const #13496u : u32
        let s_3_0: u32 = 13496;
        // D s_3_1: read-reg s_3_0:struct
        let s_3_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: call _get_BRBIDR0_EL1_Type_NUMREC(s_3_1)
        let s_3_2: u8 = u_get_BRBIDR0_EL1_Type_NUMREC(state, tracer, s_3_1);
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 8u16);
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (s_3_3.value() as i128);
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // C s_3_6: const #64u : u8
        let s_3_6: u8 = 64;
        // C s_3_7: cast zx s_3_6 -> bv
        let s_3_7: Bits = Bits::new(s_3_6 as u128, 8u16);
        // C s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (s_3_7.value() as i128);
        // C s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // D s_3_10: cast zx s_3_5 -> i
        let s_3_10: i128 = (i128::try_from(s_3_5).unwrap());
        // C s_3_11: cast zx s_3_9 -> i
        let s_3_11: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_12: cmp-eq s_3_10 s_3_11
        let s_3_12: bool = ((s_3_10) == (s_3_11));
        // D s_3_13: write-var gs#3405 <= s_3_12
        fn_state.gs_3405 = s_3_12;
        // N s_3_14: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_4_0: read-var gs#3405:u8
        let s_4_0: bool = fn_state.gs_3405;
        // D s_4_1: write-var gs#3406 <= s_4_0
        fn_state.gs_3406 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_5_0: read-var gs#3406:u8
        let s_5_0: bool = fn_state.gs_3406;
        // D s_5_1: write-var gs#3407 <= s_5_0
        fn_state.gs_3407 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_6_0: read-var gs#3407:u8
        let s_6_0: bool = fn_state.gs_3407;
        // N s_6_1: assert s_6_0
        let s_6_1: () = assert!(s_6_0);
        // C s_6_2: const #"Number of BRB records" : str
        let s_6_2: &'static str = "Number of BRB records";
        // S s_6_3: tail-call __IMPDEF_integer(s_6_2)
        return u__IMPDEF_integer(state, tracer, s_6_2);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // D s_7_1: write-var gs#3405 <= s_7_0
        fn_state.gs_3405 = s_7_0;
        // N s_7_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_8_0: const #1u : u8
        let s_8_0: bool = true;
        // D s_8_1: write-var gs#3406 <= s_8_0
        fn_state.gs_3406 = s_8_0;
        // N s_8_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var gs#3407 <= s_9_0
        fn_state.gs_3407 = s_9_0;
        // N s_9_2: jump b6
        return block_6(state, tracer, fn_state);
    }
}
