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
use IsAsyncAbort__1::*;
use IsSecondStage::*;
use AArch32_EncodeAsyncErrorSyndrome::*;
use EncodeLDFSC::*;
use IsExternalSyncAbort__1::*;
use u__UNKNOWN_bit::*;
use AArch32_PEErrorState::*;
use Bit::*;
use LSInstructionSyndrome::*;
use Zeros::*;
use u__IMPDEF_boolean::*;
use HaveRASExt::*;
use IsExternalAbort__1::*;
use common::*;
pub fn AArch32_FaultSyndrome<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d_side: bool,
    fault: ProductType1d757adad216cdef,
) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        ga_6128: bool,
        iss: u32,
        gs_8787: bool,
        gs_8785: bool,
        ga_6122: ProductType9878976b5bcce9c9,
        ga_6110: ProductType9878976b5bcce9c9,
        gs_8779: bool,
        gs_8786: bool,
        gs_8773: bool,
        ga_6096: ProductType9878976b5bcce9c9,
        ga_6100: ProductType9878976b5bcce9c9,
        gs_8774: bool,
        gs_8775: bool,
        ga_6108: ProductType9878976b5bcce9c9,
        gs_8777: bool,
        gs_8778: bool,
        gs_8776: bool,
        gs_8788: bool,
        gs_8764: bool,
        ga_6106: ProductType9878976b5bcce9c9,
        ga_6098: ProductType9878976b5bcce9c9,
        ga_6088: ProductType9878976b5bcce9c9,
        ga_6134: bool,
        d_side: bool,
        fault: ProductType1d757adad216cdef,
    }
    let fn_state = FunctionState {
        d_side,
        fault,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_0_0: read-var fault.16:struct
        let s_0_0: u32 = fn_state.fault._16;
        // C s_0_1: const #0u : u32
        let s_0_1: u32 = 0;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // N s_0_3: assert s_0_2
        let s_0_3: () = assert!(s_0_2);
        // C s_0_4: const #25s : i
        let s_0_4: i128 = 25;
        // S s_0_5: call Zeros(s_0_4)
        let s_0_5: Bits = Zeros(state, tracer, s_0_4);
        // S s_0_6: cast reint s_0_5 -> u25
        let s_0_6: u32 = (s_0_5.value() as u32);
        // D s_0_7: write-var iss <= s_0_6
        fn_state.iss = s_0_6;
        // C s_0_8: const #24s : i
        let s_0_8: i128 = 24;
        // S s_0_9: call Zeros(s_0_8)
        let s_0_9: Bits = Zeros(state, tracer, s_0_8);
        // C s_0_10: const #() : ()
        let s_0_10: () = ();
        // S s_0_11: call HaveRASExt(s_0_10)
        let s_0_11: bool = HaveRASExt(state, tracer, s_0_10);
        // N s_0_12: branch s_0_11 b63 b1
        if s_0_11 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#8764 <= s_1_0
        fn_state.gs_8764 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_2_0: read-var gs#8764:u8
        let s_2_0: bool = fn_state.gs_8764;
        // N s_2_1: branch s_2_0 b62 b3
        if s_2_0 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_4_0: read-var d_side:u8
        let s_4_0: bool = fn_state.d_side;
        // N s_4_1: branch s_4_0 b13 b5
        if s_4_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_6_0: read-var fault:struct
        let s_6_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_6_1: call IsExternalAbort__1(s_6_0)
        let s_6_1: bool = IsExternalAbort__1(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b12 b7
        if s_6_1 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_8_0: read-var fault.14:struct
        let s_8_0: bool = fn_state.fault._14;
        // N s_8_1: branch s_8_0 b11 b9
        if s_8_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var ga#6134 <= s_9_0
        fn_state.ga_6134 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_10_0: read-var ga#6134:u8
        let s_10_0: bool = fn_state.ga_6134;
        // D s_10_1: call Bit(s_10_0)
        let s_10_1: bool = Bit(state, tracer, s_10_0);
        // C s_10_2: const #7s : i
        let s_10_2: i128 = 7;
        // D s_10_3: read-var iss:u25
        let s_10_3: u32 = fn_state.iss;
        // D s_10_4: cast zx s_10_3 -> bv
        let s_10_4: Bits = Bits::new(s_10_3 as u128, 25u16);
        // C s_10_5: const #1u : u64
        let s_10_5: u64 = 1;
        // D s_10_6: bit-insert s_10_4 s_10_4 s_10_2 s_10_5
        let s_10_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_10_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_10_4.length(),
            );
            (s_10_4 & mask) | (s_10_4 << s_10_2)
        };
        // D s_10_7: cast reint s_10_6 -> u25
        let s_10_7: u32 = (s_10_6.value() as u32);
        // D s_10_8: write-var iss <= s_10_7
        fn_state.iss = s_10_7;
        // D s_10_9: read-var fault.16:struct
        let s_10_9: u32 = fn_state.fault._16;
        // D s_10_10: read-var fault.9:struct
        let s_10_10: i128 = fn_state.fault._9;
        // D s_10_11: call EncodeLDFSC(s_10_9, s_10_10)
        let s_10_11: u8 = EncodeLDFSC(state, tracer, s_10_9, s_10_10);
        // C s_10_12: const #0s : i
        let s_10_12: i128 = 0;
        // D s_10_13: read-var iss:u25
        let s_10_13: u32 = fn_state.iss;
        // D s_10_14: cast zx s_10_13 -> bv
        let s_10_14: Bits = Bits::new(s_10_13 as u128, 25u16);
        // D s_10_15: cast zx s_10_11 -> bv
        let s_10_15: Bits = Bits::new(s_10_11 as u128, 6u16);
        // C s_10_16: const #5s : i
        let s_10_16: i128 = 5;
        // C s_10_17: const #1u : u64
        let s_10_17: u64 = 1;
        // C s_10_18: cast zx s_10_17 -> bv
        let s_10_18: Bits = Bits::new(s_10_17 as u128, 64u16);
        // C s_10_19: lsl s_10_18 s_10_16
        let s_10_19: Bits = s_10_18 << s_10_16;
        // C s_10_20: sub s_10_19 s_10_18
        let s_10_20: Bits = ((s_10_19) - (s_10_18));
        // D s_10_21: and s_10_15 s_10_20
        let s_10_21: Bits = ((s_10_15) & (s_10_20));
        // D s_10_22: lsl s_10_21 s_10_12
        let s_10_22: Bits = s_10_21 << s_10_12;
        // C s_10_23: lsl s_10_20 s_10_12
        let s_10_23: Bits = s_10_20 << s_10_12;
        // C s_10_24: cmpl s_10_23
        let s_10_24: Bits = !s_10_23;
        // D s_10_25: and s_10_14 s_10_24
        let s_10_25: Bits = ((s_10_14) & (s_10_24));
        // D s_10_26: or s_10_25 s_10_22
        let s_10_26: Bits = ((s_10_25) | (s_10_22));
        // D s_10_27: cast reint s_10_26 -> u25
        let s_10_27: u32 = (s_10_26.value() as u32);
        // D s_10_28: write-var iss <= s_10_27
        fn_state.iss = s_10_27;
        // D s_10_29: read-var iss:u25
        let s_10_29: u32 = fn_state.iss;
        // N s_10_30: return s_10_29
        return s_10_29;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var ga#6134 <= s_11_0
        fn_state.ga_6134 = s_11_0;
        // N s_11_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_12_0: read-var fault.5:struct
        let s_12_0: bool = fn_state.fault._5;
        // D s_12_1: call Bit(s_12_0)
        let s_12_1: bool = Bit(state, tracer, s_12_0);
        // C s_12_2: const #9s : i
        let s_12_2: i128 = 9;
        // D s_12_3: read-var iss:u25
        let s_12_3: u32 = fn_state.iss;
        // D s_12_4: cast zx s_12_3 -> bv
        let s_12_4: Bits = Bits::new(s_12_3 as u128, 25u16);
        // C s_12_5: const #1u : u64
        let s_12_5: u64 = 1;
        // D s_12_6: bit-insert s_12_4 s_12_4 s_12_2 s_12_5
        let s_12_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_12_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_12_4.length(),
            );
            (s_12_4 & mask) | (s_12_4 << s_12_2)
        };
        // D s_12_7: cast reint s_12_6 -> u25
        let s_12_7: u32 = (s_12_6.value() as u32);
        // D s_12_8: write-var iss <= s_12_7
        fn_state.iss = s_12_7;
        // N s_12_9: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_13_0: read-var fault:struct
        let s_13_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_13_1: call IsSecondStage(s_13_0)
        let s_13_1: bool = IsSecondStage(state, tracer, s_13_0);
        // N s_13_2: branch s_13_1 b61 b14
        if s_13_1 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#8773 <= s_14_0
        fn_state.gs_8773 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_15_0: read-var gs#8773:u8
        let s_15_0: bool = fn_state.gs_8773;
        // N s_15_1: branch s_15_0 b51 b16
        if s_15_0 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#8777 <= s_16_0
        fn_state.gs_8777 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_17_0: read-var gs#8777:u8
        let s_17_0: bool = fn_state.gs_8777;
        // N s_17_1: branch s_17_0 b50 b18
        if s_17_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_18_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_19_0: read-var fault.0:struct
        let s_19_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_19_1: write-var ga#6096 <= s_19_0
        fn_state.ga_6096 = s_19_0;
        // D s_19_2: read-var ga#6096.1:struct
        let s_19_2: u32 = fn_state.ga_6096._1;
        // C s_19_3: const #6u : u32
        let s_19_3: u32 = 6;
        // D s_19_4: cmp-eq s_19_2 s_19_3
        let s_19_4: bool = ((s_19_2) == (s_19_3));
        // N s_19_5: branch s_19_4 b49 b20
        if s_19_4 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_20_0: read-var fault.0:struct
        let s_20_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_20_1: write-var ga#6098 <= s_20_0
        fn_state.ga_6098 = s_20_0;
        // D s_20_2: read-var ga#6098.1:struct
        let s_20_2: u32 = fn_state.ga_6098._1;
        // C s_20_3: const #5u : u32
        let s_20_3: u32 = 5;
        // D s_20_4: cmp-eq s_20_2 s_20_3
        let s_20_4: bool = ((s_20_2) == (s_20_3));
        // N s_20_5: branch s_20_4 b48 b21
        if s_20_4 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_21_0: read-var fault.0:struct
        let s_21_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_21_1: write-var ga#6100 <= s_21_0
        fn_state.ga_6100 = s_21_0;
        // D s_21_2: read-var ga#6100.1:struct
        let s_21_2: u32 = fn_state.ga_6100._1;
        // C s_21_3: const #8u : u32
        let s_21_3: u32 = 8;
        // D s_21_4: cmp-eq s_21_2 s_21_3
        let s_21_4: bool = ((s_21_2) == (s_21_3));
        // D s_21_5: write-var gs#8778 <= s_21_4
        fn_state.gs_8778 = s_21_4;
        // N s_21_6: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_22_0: read-var gs#8778:u8
        let s_22_0: bool = fn_state.gs_8778;
        // D s_22_1: write-var gs#8779 <= s_22_0
        fn_state.gs_8779 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_23_0: read-var gs#8779:u8
        let s_23_0: bool = fn_state.gs_8779;
        // N s_23_1: branch s_23_0 b47 b24
        if s_23_0 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_24_0: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_25_0: read-var fault.0:struct
        let s_25_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_25_1: write-var ga#6106 <= s_25_0
        fn_state.ga_6106 = s_25_0;
        // D s_25_2: read-var ga#6106.1:struct
        let s_25_2: u32 = fn_state.ga_6106._1;
        // C s_25_3: const #6u : u32
        let s_25_3: u32 = 6;
        // D s_25_4: cmp-eq s_25_2 s_25_3
        let s_25_4: bool = ((s_25_2) == (s_25_3));
        // N s_25_5: branch s_25_4 b46 b26
        if s_25_4 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_26_0: read-var fault.0:struct
        let s_26_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_26_1: write-var ga#6108 <= s_26_0
        fn_state.ga_6108 = s_26_0;
        // D s_26_2: read-var ga#6108.1:struct
        let s_26_2: u32 = fn_state.ga_6108._1;
        // C s_26_3: const #5u : u32
        let s_26_3: u32 = 5;
        // D s_26_4: cmp-eq s_26_2 s_26_3
        let s_26_4: bool = ((s_26_2) == (s_26_3));
        // N s_26_5: branch s_26_4 b45 b27
        if s_26_4 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_27_0: read-var fault.0:struct
        let s_27_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_27_1: write-var ga#6110 <= s_27_0
        fn_state.ga_6110 = s_27_0;
        // D s_27_2: read-var ga#6110.1:struct
        let s_27_2: u32 = fn_state.ga_6110._1;
        // C s_27_3: const #8u : u32
        let s_27_3: u32 = 8;
        // D s_27_4: cmp-eq s_27_2 s_27_3
        let s_27_4: bool = ((s_27_2) == (s_27_3));
        // D s_27_5: write-var gs#8785 <= s_27_4
        fn_state.gs_8785 = s_27_4;
        // N s_27_6: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_28_0: read-var gs#8785:u8
        let s_28_0: bool = fn_state.gs_8785;
        // D s_28_1: write-var gs#8786 <= s_28_0
        fn_state.gs_8786 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_29_0: read-var gs#8786:u8
        let s_29_0: bool = fn_state.gs_8786;
        // N s_29_1: branch s_29_0 b44 b30
        if s_29_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_30_0: read-var fault.16:struct
        let s_30_0: u32 = fn_state.fault._16;
        // C s_30_1: const #20u : u32
        let s_30_1: u32 = 20;
        // D s_30_2: cmp-eq s_30_0 s_30_1
        let s_30_2: bool = ((s_30_0) == (s_30_1));
        // N s_30_3: branch s_30_2 b43 b31
        if s_30_2 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_31_0: read-var fault.16:struct
        let s_31_0: u32 = fn_state.fault._16;
        // C s_31_1: const #22u : u32
        let s_31_1: u32 = 22;
        // D s_31_2: cmp-eq s_31_0 s_31_1
        let s_31_2: bool = ((s_31_0) == (s_31_1));
        // D s_31_3: write-var gs#8787 <= s_31_2
        fn_state.gs_8787 = s_31_2;
        // N s_31_4: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_32_0: read-var gs#8787:u8
        let s_32_0: bool = fn_state.gs_8787;
        // N s_32_1: branch s_32_0 b42 b33
        if s_32_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_33_0: read-var fault.0:struct
        let s_33_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_33_1: write-var ga#6122 <= s_33_0
        fn_state.ga_6122 = s_33_0;
        // D s_33_2: read-var ga#6122.4:struct
        let s_33_2: bool = fn_state.ga_6122._4;
        // N s_33_3: branch s_33_2 b41 b34
        if s_33_2 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#8788 <= s_34_0
        fn_state.gs_8788 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_35_0: read-var gs#8788:u8
        let s_35_0: bool = fn_state.gs_8788;
        // N s_35_1: branch s_35_0 b40 b36
        if s_35_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_36_0: read-var fault.19:struct
        let s_36_0: bool = fn_state.fault._19;
        // N s_36_1: branch s_36_0 b39 b37
        if s_36_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var ga#6128 <= s_37_0
        fn_state.ga_6128 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_38_0: read-var ga#6128:u8
        let s_38_0: bool = fn_state.ga_6128;
        // D s_38_1: call Bit(s_38_0)
        let s_38_1: bool = Bit(state, tracer, s_38_0);
        // C s_38_2: const #6s : i
        let s_38_2: i128 = 6;
        // D s_38_3: read-var iss:u25
        let s_38_3: u32 = fn_state.iss;
        // D s_38_4: cast zx s_38_3 -> bv
        let s_38_4: Bits = Bits::new(s_38_3 as u128, 25u16);
        // C s_38_5: const #1u : u64
        let s_38_5: u64 = 1;
        // D s_38_6: bit-insert s_38_4 s_38_4 s_38_2 s_38_5
        let s_38_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_38_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_38_4.length(),
            );
            (s_38_4 & mask) | (s_38_4 << s_38_2)
        };
        // D s_38_7: cast reint s_38_6 -> u25
        let s_38_7: u32 = (s_38_6.value() as u32);
        // D s_38_8: write-var iss <= s_38_7
        fn_state.iss = s_38_7;
        // N s_38_9: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // D s_39_1: write-var ga#6128 <= s_39_0
        fn_state.ga_6128 = s_39_0;
        // N s_39_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call __UNKNOWN_bit(s_40_0)
        let s_40_1: bool = u__UNKNOWN_bit(state, tracer, s_40_0);
        // S s_40_2: call Bit(s_40_1)
        let s_40_2: bool = Bit(state, tracer, s_40_1);
        // C s_40_3: const #6s : i
        let s_40_3: i128 = 6;
        // D s_40_4: read-var iss:u25
        let s_40_4: u32 = fn_state.iss;
        // D s_40_5: cast zx s_40_4 -> bv
        let s_40_5: Bits = Bits::new(s_40_4 as u128, 25u16);
        // C s_40_6: const #1u : u64
        let s_40_6: u64 = 1;
        // D s_40_7: bit-insert s_40_5 s_40_5 s_40_3 s_40_6
        let s_40_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_40_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_40_5.length(),
            );
            (s_40_5 & mask) | (s_40_5 << s_40_3)
        };
        // D s_40_8: cast reint s_40_7 -> u25
        let s_40_8: u32 = (s_40_7.value() as u32);
        // D s_40_9: write-var iss <= s_40_8
        fn_state.iss = s_40_8;
        // N s_40_10: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_41_0: read-var fault:struct
        let s_41_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_41_1: call IsExternalAbort__1(s_41_0)
        let s_41_1: bool = IsExternalAbort__1(state, tracer, s_41_0);
        // D s_41_2: write-var gs#8788 <= s_41_1
        fn_state.gs_8788 = s_41_1;
        // N s_41_3: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call __UNKNOWN_bit(s_42_0)
        let s_42_1: bool = u__UNKNOWN_bit(state, tracer, s_42_0);
        // S s_42_2: call Bit(s_42_1)
        let s_42_2: bool = Bit(state, tracer, s_42_1);
        // C s_42_3: const #6s : i
        let s_42_3: i128 = 6;
        // D s_42_4: read-var iss:u25
        let s_42_4: u32 = fn_state.iss;
        // D s_42_5: cast zx s_42_4 -> bv
        let s_42_5: Bits = Bits::new(s_42_4 as u128, 25u16);
        // C s_42_6: const #1u : u64
        let s_42_6: u64 = 1;
        // D s_42_7: bit-insert s_42_5 s_42_5 s_42_3 s_42_6
        let s_42_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_42_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_42_5.length(),
            );
            (s_42_5 & mask) | (s_42_5 << s_42_3)
        };
        // D s_42_8: cast reint s_42_7 -> u25
        let s_42_8: u32 = (s_42_7.value() as u32);
        // D s_42_9: write-var iss <= s_42_8
        fn_state.iss = s_42_8;
        // N s_42_10: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_43_0: const #1u : u8
        let s_43_0: bool = true;
        // D s_43_1: write-var gs#8787 <= s_43_0
        fn_state.gs_8787 = s_43_0;
        // N s_43_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_44_0: const #1u : u8
        let s_44_0: bool = true;
        // S s_44_1: call Bit(s_44_0)
        let s_44_1: bool = Bit(state, tracer, s_44_0);
        // C s_44_2: const #6s : i
        let s_44_2: i128 = 6;
        // D s_44_3: read-var iss:u25
        let s_44_3: u32 = fn_state.iss;
        // D s_44_4: cast zx s_44_3 -> bv
        let s_44_4: Bits = Bits::new(s_44_3 as u128, 25u16);
        // C s_44_5: const #1u : u64
        let s_44_5: u64 = 1;
        // D s_44_6: bit-insert s_44_4 s_44_4 s_44_2 s_44_5
        let s_44_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_44_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_44_4.length(),
            );
            (s_44_4 & mask) | (s_44_4 << s_44_2)
        };
        // D s_44_7: cast reint s_44_6 -> u25
        let s_44_7: u32 = (s_44_6.value() as u32);
        // D s_44_8: write-var iss <= s_44_7
        fn_state.iss = s_44_7;
        // N s_44_9: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_45_0: const #1u : u8
        let s_45_0: bool = true;
        // D s_45_1: write-var gs#8785 <= s_45_0
        fn_state.gs_8785 = s_45_0;
        // N s_45_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_46_0: const #1u : u8
        let s_46_0: bool = true;
        // D s_46_1: write-var gs#8786 <= s_46_0
        fn_state.gs_8786 = s_46_0;
        // N s_46_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_47_0: const #1u : u8
        let s_47_0: bool = true;
        // S s_47_1: call Bit(s_47_0)
        let s_47_1: bool = Bit(state, tracer, s_47_0);
        // C s_47_2: const #8s : i
        let s_47_2: i128 = 8;
        // D s_47_3: read-var iss:u25
        let s_47_3: u32 = fn_state.iss;
        // D s_47_4: cast zx s_47_3 -> bv
        let s_47_4: Bits = Bits::new(s_47_3 as u128, 25u16);
        // C s_47_5: const #1u : u64
        let s_47_5: u64 = 1;
        // D s_47_6: bit-insert s_47_4 s_47_4 s_47_2 s_47_5
        let s_47_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_47_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_47_4.length(),
            );
            (s_47_4 & mask) | (s_47_4 << s_47_2)
        };
        // D s_47_7: cast reint s_47_6 -> u25
        let s_47_7: u32 = (s_47_6.value() as u32);
        // D s_47_8: write-var iss <= s_47_7
        fn_state.iss = s_47_7;
        // N s_47_9: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_48_0: const #1u : u8
        let s_48_0: bool = true;
        // D s_48_1: write-var gs#8778 <= s_48_0
        fn_state.gs_8778 = s_48_0;
        // N s_48_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_49_0: const #1u : u8
        let s_49_0: bool = true;
        // D s_49_1: write-var gs#8779 <= s_49_0
        fn_state.gs_8779 = s_49_0;
        // N s_49_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_50_0: const #() : ()
        let s_50_0: () = ();
        // S s_50_1: call LSInstructionSyndrome(s_50_0)
        let s_50_1: u16 = LSInstructionSyndrome(state, tracer, s_50_0);
        // C s_50_2: const #14s : i
        let s_50_2: i128 = 14;
        // D s_50_3: read-var iss:u25
        let s_50_3: u32 = fn_state.iss;
        // D s_50_4: cast zx s_50_3 -> bv
        let s_50_4: Bits = Bits::new(s_50_3 as u128, 25u16);
        // S s_50_5: cast zx s_50_1 -> bv
        let s_50_5: Bits = Bits::new(s_50_1 as u128, 11u16);
        // C s_50_6: const #10s : i
        let s_50_6: i128 = 10;
        // C s_50_7: const #1u : u64
        let s_50_7: u64 = 1;
        // C s_50_8: cast zx s_50_7 -> bv
        let s_50_8: Bits = Bits::new(s_50_7 as u128, 64u16);
        // C s_50_9: lsl s_50_8 s_50_6
        let s_50_9: Bits = s_50_8 << s_50_6;
        // C s_50_10: sub s_50_9 s_50_8
        let s_50_10: Bits = ((s_50_9) - (s_50_8));
        // S s_50_11: and s_50_5 s_50_10
        let s_50_11: Bits = ((s_50_5) & (s_50_10));
        // S s_50_12: lsl s_50_11 s_50_2
        let s_50_12: Bits = s_50_11 << s_50_2;
        // C s_50_13: lsl s_50_10 s_50_2
        let s_50_13: Bits = s_50_10 << s_50_2;
        // C s_50_14: cmpl s_50_13
        let s_50_14: Bits = !s_50_13;
        // D s_50_15: and s_50_4 s_50_14
        let s_50_15: Bits = ((s_50_4) & (s_50_14));
        // D s_50_16: or s_50_15 s_50_12
        let s_50_16: Bits = ((s_50_15) | (s_50_12));
        // D s_50_17: cast reint s_50_16 -> u25
        let s_50_17: u32 = (s_50_16.value() as u32);
        // D s_50_18: write-var iss <= s_50_17
        fn_state.iss = s_50_17;
        // N s_50_19: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_51_0: read-var fault:struct
        let s_51_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_51_1: call IsExternalSyncAbort__1(s_51_0)
        let s_51_1: bool = IsExternalSyncAbort__1(state, tracer, s_51_0);
        // D s_51_2: not s_51_1
        let s_51_2: bool = !s_51_1;
        // N s_51_3: branch s_51_2 b60 b52
        if s_51_2 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call HaveRASExt(s_52_0)
        let s_52_1: bool = HaveRASExt(state, tracer, s_52_0);
        // S s_52_2: not s_52_1
        let s_52_2: bool = !s_52_1;
        // N s_52_3: branch s_52_2 b59 b53
        if s_52_2 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_53_0: const #0u : u8
        let s_53_0: bool = false;
        // D s_53_1: write-var gs#8774 <= s_53_0
        fn_state.gs_8774 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_54_0: read-var gs#8774:u8
        let s_54_0: bool = fn_state.gs_8774;
        // N s_54_1: branch s_54_0 b58 b55
        if s_54_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_55_0: const #0u : u8
        let s_55_0: bool = false;
        // D s_55_1: write-var gs#8775 <= s_55_0
        fn_state.gs_8775 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_56_0: read-var gs#8775:u8
        let s_56_0: bool = fn_state.gs_8775;
        // D s_56_1: write-var gs#8776 <= s_56_0
        fn_state.gs_8776 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_57_0: read-var gs#8776:u8
        let s_57_0: bool = fn_state.gs_8776;
        // D s_57_1: write-var gs#8777 <= s_57_0
        fn_state.gs_8777 = s_57_0;
        // N s_57_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_58_0: const #"ISV on second stage translation table walk" : str
        let s_58_0: &'static str = "ISV on second stage translation table walk";
        // S s_58_1: call __IMPDEF_boolean(s_58_0)
        let s_58_1: bool = u__IMPDEF_boolean(state, tracer, s_58_0);
        // D s_58_2: write-var gs#8775 <= s_58_1
        fn_state.gs_8775 = s_58_1;
        // N s_58_3: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_59_0: read-var fault.0:struct
        let s_59_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_59_1: write-var ga#6088 <= s_59_0
        fn_state.ga_6088 = s_59_0;
        // D s_59_2: read-var ga#6088.1:struct
        let s_59_2: u32 = fn_state.ga_6088._1;
        // C s_59_3: const #13u : u32
        let s_59_3: u32 = 13;
        // D s_59_4: cmp-eq s_59_2 s_59_3
        let s_59_4: bool = ((s_59_2) == (s_59_3));
        // D s_59_5: write-var gs#8774 <= s_59_4
        fn_state.gs_8774 = s_59_4;
        // N s_59_6: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_60_0: const #1u : u8
        let s_60_0: bool = true;
        // D s_60_1: write-var gs#8776 <= s_60_0
        fn_state.gs_8776 = s_60_0;
        // N s_60_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_61_0: read-var fault.14:struct
        let s_61_0: bool = fn_state.fault._14;
        // D s_61_1: not s_61_0
        let s_61_1: bool = !s_61_0;
        // D s_61_2: write-var gs#8773 <= s_61_1
        fn_state.gs_8773 = s_61_1;
        // N s_61_3: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_62_0: read-var fault:struct
        let s_62_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_62_1: call AArch32_PEErrorState(s_62_0)
        let s_62_1: u32 = AArch32_PEErrorState(state, tracer, s_62_0);
        // D s_62_2: call AArch32_EncodeAsyncErrorSyndrome(s_62_1)
        let s_62_2: u8 = AArch32_EncodeAsyncErrorSyndrome(state, tracer, s_62_1);
        // C s_62_3: const #10s : i
        let s_62_3: i128 = 10;
        // D s_62_4: read-var iss:u25
        let s_62_4: u32 = fn_state.iss;
        // D s_62_5: cast zx s_62_4 -> bv
        let s_62_5: Bits = Bits::new(s_62_4 as u128, 25u16);
        // D s_62_6: cast zx s_62_2 -> bv
        let s_62_6: Bits = Bits::new(s_62_2 as u128, 2u16);
        // C s_62_7: const #1s : i
        let s_62_7: i128 = 1;
        // C s_62_8: const #1u : u64
        let s_62_8: u64 = 1;
        // C s_62_9: cast zx s_62_8 -> bv
        let s_62_9: Bits = Bits::new(s_62_8 as u128, 64u16);
        // C s_62_10: lsl s_62_9 s_62_7
        let s_62_10: Bits = s_62_9 << s_62_7;
        // C s_62_11: sub s_62_10 s_62_9
        let s_62_11: Bits = ((s_62_10) - (s_62_9));
        // D s_62_12: and s_62_6 s_62_11
        let s_62_12: Bits = ((s_62_6) & (s_62_11));
        // D s_62_13: lsl s_62_12 s_62_3
        let s_62_13: Bits = s_62_12 << s_62_3;
        // C s_62_14: lsl s_62_11 s_62_3
        let s_62_14: Bits = s_62_11 << s_62_3;
        // C s_62_15: cmpl s_62_14
        let s_62_15: Bits = !s_62_14;
        // D s_62_16: and s_62_5 s_62_15
        let s_62_16: Bits = ((s_62_5) & (s_62_15));
        // D s_62_17: or s_62_16 s_62_13
        let s_62_17: Bits = ((s_62_16) | (s_62_13));
        // D s_62_18: cast reint s_62_17 -> u25
        let s_62_18: u32 = (s_62_17.value() as u32);
        // D s_62_19: write-var iss <= s_62_18
        fn_state.iss = s_62_18;
        // N s_62_20: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_63_0: read-var fault:struct
        let s_63_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_63_1: call IsAsyncAbort__1(s_63_0)
        let s_63_1: bool = IsAsyncAbort__1(state, tracer, s_63_0);
        // D s_63_2: write-var gs#8764 <= s_63_1
        fn_state.gs_8764 = s_63_1;
        // N s_63_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
