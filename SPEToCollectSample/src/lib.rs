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
use neq_int::*;
use IsZero::*;
use SPEGetRandomInterval::*;
use SPEResetSampleCounter::*;
use u_get_PMSICR_EL1_Type_COUNT::*;
use u_get_PMSICR_EL1_Type_ECOUNT::*;
use u_get_PMSIRR_EL1_Type_RND::*;
use u_get_PMSIDR_EL1_Type_ERnd::*;
use common::*;
pub fn SPEToCollectSample<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_25661: (),
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        return_value: bool,
        gs_25676: bool,
        gs_25661: (),
    }
    let fn_state = FunctionState {
        gs_25661,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #101816u : u32
        let s_0_0: u32 = 101816;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call _get_PMSICR_EL1_Type_COUNT(s_0_1)
        let s_0_2: u32 = u_get_PMSICR_EL1_Type_COUNT(state, tracer, s_0_1);
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_4: call IsZero(s_0_3)
        let s_0_4: bool = IsZero(state, tracer, s_0_3);
        // N s_0_5: branch s_0_4 b17 b1
        if s_0_4 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #101816u : u32
        let s_1_0: u32 = 101816;
        // D s_1_1: read-reg s_1_0:struct
        let s_1_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: call _get_PMSICR_EL1_Type_COUNT(s_1_1)
        let s_1_2: u32 = u_get_PMSICR_EL1_Type_COUNT(state, tracer, s_1_1);
        // C s_1_3: const #101816u : u32
        let s_1_3: u32 = 101816;
        // D s_1_4: read-reg s_1_3:struct
        let s_1_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // C s_1_5: const #101816u : u32
        let s_1_5: u32 = 101816;
        // N s_1_6: write-reg s_1_5 <= s_1_4
        let s_1_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_1_5 as isize, s_1_4);
            tracer.write_register(s_1_5 as isize, s_1_4);
        };
        // C s_1_7: const #101816u : u32
        let s_1_7: u32 = 101816;
        // D s_1_8: read-reg s_1_7:struct
        let s_1_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_1_7 as isize);
            tracer.read_register(s_1_7 as isize, value);
            value
        };
        // D s_1_9: call _get_PMSICR_EL1_Type_COUNT(s_1_8)
        let s_1_9: u32 = u_get_PMSICR_EL1_Type_COUNT(state, tracer, s_1_8);
        // D s_1_10: cast zx s_1_9 -> bv
        let s_1_10: Bits = Bits::new(s_1_9 as u128, 32u16);
        // D s_1_11: call IsZero(s_1_10)
        let s_1_11: bool = IsZero(state, tracer, s_1_10);
        // N s_1_12: branch s_1_11 b11 b2
        if s_1_11 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_2_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #101816u : u32
        let s_4_0: u32 = 101816;
        // D s_4_1: read-reg s_4_0:struct
        let s_4_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: call _get_PMSICR_EL1_Type_ECOUNT(s_4_1)
        let s_4_2: u8 = u_get_PMSICR_EL1_Type_ECOUNT(state, tracer, s_4_1);
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 8u16);
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (s_4_3.value() as i128);
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // C s_4_6: const #0s : i
        let s_4_6: i128 = 0;
        // D s_4_7: cast zx s_4_5 -> i
        let s_4_7: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_8: call neq_int(s_4_7, s_4_6)
        let s_4_8: bool = neq_int(state, tracer, s_4_7, s_4_6);
        // N s_4_9: branch s_4_8 b8 b5
        if s_4_8 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var return_value <= s_6_0
        fn_state.return_value = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var return_value:u8
        let s_7_0: bool = fn_state.return_value;
        // N s_7_1: return s_7_0
        return s_7_0;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #101816u : u32
        let s_8_0: u32 = 101816;
        // D s_8_1: read-reg s_8_0:struct
        let s_8_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: call _get_PMSICR_EL1_Type_ECOUNT(s_8_1)
        let s_8_2: u8 = u_get_PMSICR_EL1_Type_ECOUNT(state, tracer, s_8_1);
        // C s_8_3: const #101816u : u32
        let s_8_3: u32 = 101816;
        // D s_8_4: read-reg s_8_3:struct
        let s_8_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_3 as isize);
            tracer.read_register(s_8_3 as isize, value);
            value
        };
        // C s_8_5: const #101816u : u32
        let s_8_5: u32 = 101816;
        // N s_8_6: write-reg s_8_5 <= s_8_4
        let s_8_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_8_5 as isize, s_8_4);
            tracer.write_register(s_8_5 as isize, s_8_4);
        };
        // C s_8_7: const #101816u : u32
        let s_8_7: u32 = 101816;
        // D s_8_8: read-reg s_8_7:struct
        let s_8_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_7 as isize);
            tracer.read_register(s_8_7 as isize, value);
            value
        };
        // D s_8_9: call _get_PMSICR_EL1_Type_ECOUNT(s_8_8)
        let s_8_9: u8 = u_get_PMSICR_EL1_Type_ECOUNT(state, tracer, s_8_8);
        // D s_8_10: cast zx s_8_9 -> bv
        let s_8_10: Bits = Bits::new(s_8_9 as u128, 8u16);
        // D s_8_11: call IsZero(s_8_10)
        let s_8_11: bool = IsZero(state, tracer, s_8_10);
        // N s_8_12: branch s_8_11 b10 b9
        if s_8_11 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_9_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var return_value <= s_10_0
        fn_state.return_value = s_10_0;
        // N s_10_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_11_0: const #12736u : u32
        let s_11_0: u32 = 12736;
        // D s_11_1: read-reg s_11_0:struct
        let s_11_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: call _get_PMSIRR_EL1_Type_RND(s_11_1)
        let s_11_2: bool = u_get_PMSIRR_EL1_Type_RND(state, tracer, s_11_1);
        // D s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // C s_11_4: const #1u : u8
        let s_11_4: bool = true;
        // C s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 1u16);
        // D s_11_6: cmp-eq s_11_3 s_11_5
        let s_11_6: bool = ((s_11_3) == (s_11_5));
        // N s_11_7: branch s_11_6 b16 b12
        if s_11_6 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#25676 <= s_12_0
        fn_state.gs_25676 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var gs#25676:u8
        let s_13_0: bool = fn_state.gs_25676;
        // N s_13_1: branch s_13_0 b15 b14
        if s_13_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var return_value <= s_14_0
        fn_state.return_value = s_14_0;
        // N s_14_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call SPEGetRandomInterval(s_15_0)
        let s_15_1: u8 = SPEGetRandomInterval(state, tracer, s_15_0);
        // C s_15_2: const #101816u : u32
        let s_15_2: u32 = 101816;
        // D s_15_3: read-reg s_15_2:struct
        let s_15_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_15_2 as isize);
            tracer.read_register(s_15_2 as isize, value);
            value
        };
        // C s_15_4: const #101816u : u32
        let s_15_4: u32 = 101816;
        // N s_15_5: write-reg s_15_4 <= s_15_3
        let s_15_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_15_4 as isize, s_15_3);
            tracer.write_register(s_15_4 as isize, s_15_3);
        };
        // N s_15_6: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_16_0: const #14736u : u32
        let s_16_0: u32 = 14736;
        // D s_16_1: read-reg s_16_0:struct
        let s_16_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: call _get_PMSIDR_EL1_Type_ERnd(s_16_1)
        let s_16_2: bool = u_get_PMSIDR_EL1_Type_ERnd(state, tracer, s_16_1);
        // D s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 1u16);
        // C s_16_4: const #1u : u8
        let s_16_4: bool = true;
        // C s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 1u16);
        // D s_16_6: cmp-eq s_16_3 s_16_5
        let s_16_6: bool = ((s_16_3) == (s_16_5));
        // D s_16_7: write-var gs#25676 <= s_16_6
        fn_state.gs_25676 = s_16_6;
        // N s_16_8: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call SPEResetSampleCounter(s_17_0)
        let s_17_1: () = SPEResetSampleCounter(state, tracer, s_17_0);
        // N s_17_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
