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
use u_get_EDSCR2_Type_EHBWE::*;
use u_get_DBGBCR_EL1_Type_E::*;
use SelfHostedExtendedBPWPEnabled::*;
use HaltOnBreakpointOrWatchpoint::*;
use EDSCR2_read::*;
use common::*;
pub fn IsBreakpointEnabled<T: Tracer>(state: &mut State, tracer: &T, n: i64) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_16037: bool,
        gs_16038: bool,
        gs_16039: bool,
        gs_16036: bool,
        return_value: bool,
        n: i64,
    }
    let fn_state = FunctionState {
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #15s : i
        let s_0_0: i128 = 15;
        // D s_0_1: read-var n:i64
        let s_0_1: i64 = fn_state.n;
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: cmp-gt s_0_2 s_0_0
        let s_0_3: bool = ((s_0_2) > (s_0_0));
        // N s_0_4: branch s_0_3 b6 b1
        if s_0_3 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#16039 <= s_1_0
        fn_state.gs_16039 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#16039:u8
        let s_2_0: bool = fn_state.gs_16039;
        // N s_2_1: branch s_2_0 b5 b3
        if s_2_0 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #12184u : u32
        let s_3_0: u32 = 12184;
        // D s_3_1: read-reg s_3_0:[struct; 64]
        let s_3_1: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 64usize]>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: read-var n:i64
        let s_3_2: i64 = fn_state.n;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: read-element s_3_1[s_3_3]
        let s_3_4: ProductType5c790c8ef59cc8b2 = s_3_1[(s_3_3) as usize];
        // D s_3_5: call _get_DBGBCR_EL1_Type_E(s_3_4)
        let s_3_5: bool = u_get_DBGBCR_EL1_Type_E(state, tracer, s_3_4);
        // D s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 1u16);
        // C s_3_7: const #1u : u8
        let s_3_7: bool = true;
        // C s_3_8: cast zx s_3_7 -> bv
        let s_3_8: Bits = Bits::new(s_3_7 as u128, 1u16);
        // D s_3_9: cmp-eq s_3_6 s_3_8
        let s_3_9: bool = ((s_3_6) == (s_3_8));
        // D s_3_10: write-var return_value <= s_3_9
        fn_state.return_value = s_3_9;
        // N s_3_11: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var return_value:u8
        let s_4_0: bool = fn_state.return_value;
        // N s_4_1: return s_4_0
        return s_4_0;
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
        // N s_5_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call HaltOnBreakpointOrWatchpoint(s_6_0)
        let s_6_1: bool = HaltOnBreakpointOrWatchpoint(state, tracer, s_6_0);
        // S s_6_2: not s_6_1
        let s_6_2: bool = !s_6_1;
        // N s_6_3: branch s_6_2 b15 b7
        if s_6_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#16036 <= s_7_0
        fn_state.gs_16036 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var gs#16036:u8
        let s_8_0: bool = fn_state.gs_16036;
        // N s_8_1: branch s_8_0 b14 b9
        if s_8_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call HaltOnBreakpointOrWatchpoint(s_9_0)
        let s_9_1: bool = HaltOnBreakpointOrWatchpoint(state, tracer, s_9_0);
        // N s_9_2: branch s_9_1 b13 b10
        if s_9_1 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#16037 <= s_10_0
        fn_state.gs_16037 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var gs#16037:u8
        let s_11_0: bool = fn_state.gs_16037;
        // D s_11_1: write-var gs#16038 <= s_11_0
        fn_state.gs_16038 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var gs#16038:u8
        let s_12_0: bool = fn_state.gs_16038;
        // D s_12_1: write-var gs#16039 <= s_12_0
        fn_state.gs_16039 = s_12_0;
        // N s_12_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call EDSCR2_read(s_13_0)
        let s_13_1: ProductType700c18a878c5601b = EDSCR2_read(state, tracer, s_13_0);
        // S s_13_2: call _get_EDSCR2_Type_EHBWE(s_13_1)
        let s_13_2: bool = u_get_EDSCR2_Type_EHBWE(state, tracer, s_13_1);
        // S s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // C s_13_4: const #0u : u8
        let s_13_4: bool = false;
        // C s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 1u16);
        // S s_13_6: cmp-eq s_13_3 s_13_5
        let s_13_6: bool = ((s_13_3) == (s_13_5));
        // D s_13_7: write-var gs#16037 <= s_13_6
        fn_state.gs_16037 = s_13_6;
        // N s_13_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#16038 <= s_14_0
        fn_state.gs_16038 = s_14_0;
        // N s_14_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call SelfHostedExtendedBPWPEnabled(s_15_0)
        let s_15_1: bool = SelfHostedExtendedBPWPEnabled(state, tracer, s_15_0);
        // S s_15_2: not s_15_1
        let s_15_2: bool = !s_15_1;
        // D s_15_3: write-var gs#16036 <= s_15_2
        fn_state.gs_16036 = s_15_2;
        // N s_15_4: jump b8
        return block_8(state, tracer, fn_state);
    }
}
