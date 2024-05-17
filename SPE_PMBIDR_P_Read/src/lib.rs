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
use CurrentSecurityState::*;
use ProfilingBufferOwner::*;
use common::*;
pub fn SPE_PMBIDR_P_Read<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_26260: (),
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_26266: bool,
        owning_ss: u32,
        gs_26265: bool,
        return_value: bool,
        ga_20493: ProductTypec8897aad3eb4a29e,
        gs_26260: (),
    }
    let fn_state = FunctionState {
        gs_26260,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call ProfilingBufferOwner(s_0_0)
        let s_0_1: ProductTypec8897aad3eb4a29e = ProfilingBufferOwner(
            state,
            tracer,
            s_0_0,
        );
        // D s_0_2: write-var ga#20493 <= s_0_1
        fn_state.ga_20493 = s_0_1;
        // D s_0_3: read-var ga#20493.0:struct
        let s_0_3: u32 = fn_state.ga_20493._0;
        // D s_0_4: read-var ga#20493.1:struct
        let s_0_4: u8 = fn_state.ga_20493._1;
        // D s_0_5: write-var owning_ss <= s_0_3
        fn_state.owning_ss = s_0_3;
        // D s_0_6: cast zx s_0_4 -> bv
        let s_0_6: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (s_0_6.value() as i128);
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // C s_0_9: const #16975u : u32
        let s_0_9: u32 = 16975;
        // D s_0_10: read-reg s_0_9:u8
        let s_0_10: u8 = {
            let value = state.read_register::<u8>(s_0_9 as isize);
            tracer.read_register(s_0_9 as isize, value);
            value
        };
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 2u16);
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (s_0_11.value() as i128);
        // D s_0_13: cast reint s_0_12 -> i64
        let s_0_13: i64 = (s_0_12 as i64);
        // D s_0_14: cast zx s_0_8 -> i
        let s_0_14: i128 = (i128::try_from(s_0_8).unwrap());
        // D s_0_15: cast zx s_0_13 -> i
        let s_0_15: i128 = (i128::try_from(s_0_13).unwrap());
        // D s_0_16: cmp-gt s_0_14 s_0_15
        let s_0_16: bool = ((s_0_14) > (s_0_15));
        // N s_0_17: branch s_0_16 b9 b1
        if s_0_16 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 2u16);
        // C s_1_3: const #424u : u32
        let s_1_3: u32 = 424;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 2u16);
        // D s_1_6: cmp-ne s_1_2 s_1_5
        let s_1_6: bool = ((s_1_2) != (s_1_5));
        // N s_1_7: branch s_1_6 b8 b2
        if s_1_6 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#26265 <= s_2_0
        fn_state.gs_26265 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var gs#26265:u8
        let s_3_0: bool = fn_state.gs_26265;
        // D s_3_1: write-var gs#26266 <= s_3_0
        fn_state.gs_26266 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var gs#26266:u8
        let s_4_0: bool = fn_state.gs_26266;
        // N s_4_1: branch s_4_0 b7 b5
        if s_4_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var return_value <= s_5_0
        fn_state.return_value = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var return_value:u8
        let s_6_0: bool = fn_state.return_value;
        // N s_6_1: return s_6_0
        return s_6_0;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // D s_7_1: write-var return_value <= s_7_0
        fn_state.return_value = s_7_0;
        // N s_7_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call CurrentSecurityState(s_8_0)
        let s_8_1: u32 = CurrentSecurityState(state, tracer, s_8_0);
        // D s_8_2: read-var owning_ss:u32
        let s_8_2: u32 = fn_state.owning_ss;
        // D s_8_3: cmp-eq s_8_2 s_8_1
        let s_8_3: bool = ((s_8_2) == (s_8_1));
        // D s_8_4: write-var gs#26265 <= s_8_3
        fn_state.gs_26265 = s_8_3;
        // N s_8_5: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var gs#26266 <= s_9_0
        fn_state.gs_26266 = s_9_0;
        // N s_9_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
