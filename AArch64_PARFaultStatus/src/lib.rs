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
use EncodeLDFSC::*;
use common::*;
pub fn AArch64_PARFaultStatus<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault: ProductType1d757adad216cdef,
) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        fst: u8,
        ga_22435: u8,
        gs_29046: bool,
        fault: ProductType1d757adad216cdef,
    }
    let fn_state = FunctionState {
        fault,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_0_0: read-var fault.16:struct
        let s_0_0: u32 = fn_state.fault._16;
        // C s_0_1: const #4u : u32
        let s_0_1: u32 = 4;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // N s_0_3: branch s_0_2 b3 b1
        if s_0_2 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_1_0: read-var fault.16:struct
        let s_1_0: u32 = fn_state.fault._16;
        // D s_1_1: read-var fault.9:struct
        let s_1_1: i128 = fn_state.fault._9;
        // D s_1_2: call EncodeLDFSC(s_1_0, s_1_1)
        let s_1_2: u8 = EncodeLDFSC(state, tracer, s_1_0, s_1_1);
        // D s_1_3: write-var fst <= s_1_2
        fn_state.fst = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_2_0: read-var fst:u8
        let s_2_0: u8 = fn_state.fst;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_3_0: read-var fault.9:struct
        let s_3_0: i128 = fn_state.fault._9;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // N s_3_3: branch s_3_2 b9 b4
        if s_3_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_4_0: read-var fault.9:struct
        let s_4_0: i128 = fn_state.fault._9;
        // C s_4_1: const #2s : i
        let s_4_1: i128 = 2;
        // D s_4_2: cmp-eq s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) == (s_4_1));
        // D s_4_3: write-var gs#29046 <= s_4_2
        fn_state.gs_29046 = s_4_2;
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_5_0: read-var gs#29046:u8
        let s_5_0: bool = fn_state.gs_29046;
        // N s_5_1: assert s_5_0
        let s_5_1: () = assert!(s_5_0);
        // D s_5_2: read-var fault.9:struct
        let s_5_2: i128 = fn_state.fault._9;
        // C s_5_3: const #1s : i
        let s_5_3: i128 = 1;
        // D s_5_4: cmp-eq s_5_2 s_5_3
        let s_5_4: bool = ((s_5_2) == (s_5_3));
        // N s_5_5: branch s_5_4 b8 b6
        if s_5_4 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_6_0: const #2u : u8
        let s_6_0: u8 = 2;
        // D s_6_1: write-var ga#22435 <= s_6_0
        fn_state.ga_22435 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_7_0: const #0s : i
        let s_7_0: i128 = 0;
        // D s_7_1: read-var fst:u8
        let s_7_1: u8 = fn_state.fst;
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 6u16);
        // D s_7_3: read-var ga#22435:u8
        let s_7_3: u8 = fn_state.ga_22435;
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 2u16);
        // C s_7_5: const #1s : i
        let s_7_5: i128 = 1;
        // C s_7_6: const #1u : u64
        let s_7_6: u64 = 1;
        // C s_7_7: cast zx s_7_6 -> bv
        let s_7_7: Bits = Bits::new(s_7_6 as u128, 64u16);
        // C s_7_8: lsl s_7_7 s_7_5
        let s_7_8: Bits = s_7_7 << s_7_5;
        // C s_7_9: sub s_7_8 s_7_7
        let s_7_9: Bits = ((s_7_8) - (s_7_7));
        // D s_7_10: and s_7_4 s_7_9
        let s_7_10: Bits = ((s_7_4) & (s_7_9));
        // D s_7_11: lsl s_7_10 s_7_0
        let s_7_11: Bits = s_7_10 << s_7_0;
        // C s_7_12: lsl s_7_9 s_7_0
        let s_7_12: Bits = s_7_9 << s_7_0;
        // C s_7_13: cmpl s_7_12
        let s_7_13: Bits = !s_7_12;
        // D s_7_14: and s_7_2 s_7_13
        let s_7_14: Bits = ((s_7_2) & (s_7_13));
        // D s_7_15: or s_7_14 s_7_11
        let s_7_15: Bits = ((s_7_14) | (s_7_11));
        // D s_7_16: cast reint s_7_15 -> u8
        let s_7_16: u8 = (s_7_15.value() as u8);
        // D s_7_17: write-var fst <= s_7_16
        fn_state.fst = s_7_16;
        // C s_7_18: const #2s : i
        let s_7_18: i128 = 2;
        // D s_7_19: read-var fst:u8
        let s_7_19: u8 = fn_state.fst;
        // D s_7_20: cast zx s_7_19 -> bv
        let s_7_20: Bits = Bits::new(s_7_19 as u128, 6u16);
        // C s_7_21: const #15u : u8
        let s_7_21: u8 = 15;
        // C s_7_22: cast zx s_7_21 -> bv
        let s_7_22: Bits = Bits::new(s_7_21 as u128, 4u16);
        // C s_7_23: const #3s : i
        let s_7_23: i128 = 3;
        // C s_7_24: const #1u : u64
        let s_7_24: u64 = 1;
        // C s_7_25: cast zx s_7_24 -> bv
        let s_7_25: Bits = Bits::new(s_7_24 as u128, 64u16);
        // C s_7_26: lsl s_7_25 s_7_23
        let s_7_26: Bits = s_7_25 << s_7_23;
        // C s_7_27: sub s_7_26 s_7_25
        let s_7_27: Bits = ((s_7_26) - (s_7_25));
        // C s_7_28: and s_7_22 s_7_27
        let s_7_28: Bits = ((s_7_22) & (s_7_27));
        // C s_7_29: lsl s_7_28 s_7_18
        let s_7_29: Bits = s_7_28 << s_7_18;
        // C s_7_30: lsl s_7_27 s_7_18
        let s_7_30: Bits = s_7_27 << s_7_18;
        // C s_7_31: cmpl s_7_30
        let s_7_31: Bits = !s_7_30;
        // D s_7_32: and s_7_20 s_7_31
        let s_7_32: Bits = ((s_7_20) & (s_7_31));
        // D s_7_33: or s_7_32 s_7_29
        let s_7_33: Bits = ((s_7_32) | (s_7_29));
        // D s_7_34: cast reint s_7_33 -> u8
        let s_7_34: u8 = (s_7_33.value() as u8);
        // D s_7_35: write-var fst <= s_7_34
        fn_state.fst = s_7_34;
        // N s_7_36: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_8_0: const #1u : u8
        let s_8_0: u8 = 1;
        // D s_8_1: write-var ga#22435 <= s_8_0
        fn_state.ga_22435 = s_8_0;
        // N s_8_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var gs#29046 <= s_9_0
        fn_state.gs_29046 = s_9_0;
        // N s_9_2: jump b5
        return block_5(state, tracer, fn_state);
    }
}
