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
use HaveBlockBBM::*;
use AArch64_BlockBBMSupportLevel::*;
use u__IMPDEF_boolean::*;
use common::*;
pub fn AArch64_BlocknTFaults<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d128: bool,
    descriptor: Bits,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        nT_faults: bool,
        bbm_level: i128,
        return_value: bool,
        gs_17532: bool,
        nT: bool,
        gs_17534: bool,
        gs_17533: bool,
        d128: bool,
        descriptor: Bits,
    }
    let fn_state = FunctionState {
        d128,
        descriptor,
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
        // S s_0_1: call HaveBlockBBM(s_0_0)
        let s_0_1: bool = HaveBlockBBM(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b15 b1
        if s_0_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_1_0: read-var d128:u8
        let s_1_0: bool = fn_state.d128;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 1u16);
        // C s_1_2: const #1u : u8
        let s_1_2: bool = true;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 1u16);
        // D s_1_4: cmp-eq s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) == (s_1_3));
        // N s_1_5: branch s_1_4 b14 b2
        if s_1_4 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var descriptor:bv
        let s_2_0: Bits = fn_state.descriptor;
        // D s_2_1: size-of s_2_0
        let s_2_1: u16 = s_2_0.length();
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // C s_2_3: const #16s : i
        let s_2_3: i128 = 16;
        // D s_2_4: cmp-lt s_2_3 s_2_2
        let s_2_4: bool = ((s_2_3) < (s_2_2));
        // N s_2_5: assert s_2_4
        let s_2_5: () = assert!(s_2_4);
        // C s_2_6: const #16s : i
        let s_2_6: i128 = 16;
        // D s_2_7: read-var descriptor:bv
        let s_2_7: Bits = fn_state.descriptor;
        // C s_2_8: const #1u : u64
        let s_2_8: u64 = 1;
        // D s_2_9: bit-extract s_2_7 s_2_6 s_2_8
        let s_2_9: Bits = (Bits::new(
            ((s_2_7) >> (s_2_6)).value(),
            u16::try_from(s_2_8).unwrap(),
        ));
        // D s_2_10: cast reint s_2_9 -> u8
        let s_2_10: bool = ((s_2_9.value()) != 0);
        // C s_2_11: const #0s : i
        let s_2_11: i128 = 0;
        // C s_2_12: const #0u : u64
        let s_2_12: u64 = 0;
        // D s_2_13: cast zx s_2_10 -> u64
        let s_2_13: u64 = (s_2_10 as u64);
        // C s_2_14: const #1u : u64
        let s_2_14: u64 = 1;
        // D s_2_15: and s_2_13 s_2_14
        let s_2_15: u64 = ((s_2_13) & (s_2_14));
        // D s_2_16: cmp-eq s_2_15 s_2_14
        let s_2_16: bool = ((s_2_15) == (s_2_14));
        // D s_2_17: lsl s_2_13 s_2_11
        let s_2_17: u64 = s_2_13 << s_2_11;
        // D s_2_18: or s_2_12 s_2_17
        let s_2_18: u64 = ((s_2_12) | (s_2_17));
        // D s_2_19: cmpl s_2_17
        let s_2_19: u64 = !s_2_17;
        // D s_2_20: and s_2_12 s_2_19
        let s_2_20: u64 = ((s_2_12) & (s_2_19));
        // D s_2_21: select s_2_16 s_2_18 s_2_20
        let s_2_21: u64 = if s_2_16 { s_2_18 } else { s_2_20 };
        // D s_2_22: cast trunc s_2_21 -> u8
        let s_2_22: bool = ((s_2_21) != 0);
        // D s_2_23: write-var nT <= s_2_22
        fn_state.nT = s_2_22;
        // N s_2_24: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call AArch64_BlockBBMSupportLevel(s_3_0)
        let s_3_1: i128 = AArch64_BlockBBMSupportLevel(state, tracer, s_3_0);
        // D s_3_2: write-var bbm_level <= s_3_1
        fn_state.bbm_level = s_3_1;
        // C s_3_3: const #"BBM level 1 or 2 support nT bit causes Translation Fault" : str
        let s_3_3: &'static str = "BBM level 1 or 2 support nT bit causes Translation Fault";
        // S s_3_4: call __IMPDEF_boolean(s_3_3)
        let s_3_4: bool = u__IMPDEF_boolean(state, tracer, s_3_3);
        // D s_3_5: write-var nT_faults <= s_3_4
        fn_state.nT_faults = s_3_4;
        // C s_3_6: const #1s : i
        let s_3_6: i128 = 1;
        // D s_3_7: read-var bbm_level:i
        let s_3_7: i128 = fn_state.bbm_level;
        // D s_3_8: cmp-eq s_3_7 s_3_6
        let s_3_8: bool = ((s_3_7) == (s_3_6));
        // N s_3_9: branch s_3_8 b13 b4
        if s_3_8 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #2s : i
        let s_4_0: i128 = 2;
        // D s_4_1: read-var bbm_level:i
        let s_4_1: i128 = fn_state.bbm_level;
        // D s_4_2: cmp-eq s_4_1 s_4_0
        let s_4_2: bool = ((s_4_1) == (s_4_0));
        // D s_4_3: write-var gs#17532 <= s_4_2
        fn_state.gs_17532 = s_4_2;
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var gs#17532:u8
        let s_5_0: bool = fn_state.gs_17532;
        // N s_5_1: branch s_5_0 b12 b6
        if s_5_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#17533 <= s_6_0
        fn_state.gs_17533 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var gs#17533:u8
        let s_7_0: bool = fn_state.gs_17533;
        // N s_7_1: branch s_7_0 b11 b8
        if s_7_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#17534 <= s_8_0
        fn_state.gs_17534 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var gs#17534:u8
        let s_9_0: bool = fn_state.gs_17534;
        // D s_9_1: write-var return_value <= s_9_0
        fn_state.return_value = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var return_value:u8
        let s_10_0: bool = fn_state.return_value;
        // N s_10_1: return s_10_0
        return s_10_0;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var nT_faults:u8
        let s_11_0: bool = fn_state.nT_faults;
        // D s_11_1: write-var gs#17534 <= s_11_0
        fn_state.gs_17534 = s_11_0;
        // N s_11_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var nT:u8
        let s_12_0: bool = fn_state.nT;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 1u16);
        // C s_12_2: const #1u : u8
        let s_12_2: bool = true;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: write-var gs#17533 <= s_12_4
        fn_state.gs_17533 = s_12_4;
        // N s_12_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#17532 <= s_13_0
        fn_state.gs_17532 = s_13_0;
        // N s_13_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_14_0: read-var descriptor:bv
        let s_14_0: Bits = fn_state.descriptor;
        // D s_14_1: size-of s_14_0
        let s_14_1: u16 = s_14_0.length();
        // D s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (i128::try_from(s_14_1).unwrap());
        // C s_14_3: const #6s : i
        let s_14_3: i128 = 6;
        // D s_14_4: cmp-lt s_14_3 s_14_2
        let s_14_4: bool = ((s_14_3) < (s_14_2));
        // N s_14_5: assert s_14_4
        let s_14_5: () = assert!(s_14_4);
        // C s_14_6: const #6s : i
        let s_14_6: i128 = 6;
        // D s_14_7: read-var descriptor:bv
        let s_14_7: Bits = fn_state.descriptor;
        // C s_14_8: const #1u : u64
        let s_14_8: u64 = 1;
        // D s_14_9: bit-extract s_14_7 s_14_6 s_14_8
        let s_14_9: Bits = (Bits::new(
            ((s_14_7) >> (s_14_6)).value(),
            u16::try_from(s_14_8).unwrap(),
        ));
        // D s_14_10: cast reint s_14_9 -> u8
        let s_14_10: bool = ((s_14_9.value()) != 0);
        // C s_14_11: const #0s : i
        let s_14_11: i128 = 0;
        // C s_14_12: const #0u : u64
        let s_14_12: u64 = 0;
        // D s_14_13: cast zx s_14_10 -> u64
        let s_14_13: u64 = (s_14_10 as u64);
        // C s_14_14: const #1u : u64
        let s_14_14: u64 = 1;
        // D s_14_15: and s_14_13 s_14_14
        let s_14_15: u64 = ((s_14_13) & (s_14_14));
        // D s_14_16: cmp-eq s_14_15 s_14_14
        let s_14_16: bool = ((s_14_15) == (s_14_14));
        // D s_14_17: lsl s_14_13 s_14_11
        let s_14_17: u64 = s_14_13 << s_14_11;
        // D s_14_18: or s_14_12 s_14_17
        let s_14_18: u64 = ((s_14_12) | (s_14_17));
        // D s_14_19: cmpl s_14_17
        let s_14_19: u64 = !s_14_17;
        // D s_14_20: and s_14_12 s_14_19
        let s_14_20: u64 = ((s_14_12) & (s_14_19));
        // D s_14_21: select s_14_16 s_14_18 s_14_20
        let s_14_21: u64 = if s_14_16 { s_14_18 } else { s_14_20 };
        // D s_14_22: cast trunc s_14_21 -> u8
        let s_14_22: bool = ((s_14_21) != 0);
        // D s_14_23: write-var nT <= s_14_22
        fn_state.nT = s_14_22;
        // N s_14_24: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var return_value <= s_15_0
        fn_state.return_value = s_15_0;
        // N s_15_2: jump b10
        return block_10(state, tracer, fn_state);
    }
}
