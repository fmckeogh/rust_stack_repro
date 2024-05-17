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
use LS64InstructionSyndrome::*;
use IsExternalSyncAbort__1::*;
use u__UNKNOWN_bit::*;
use LSInstructionSyndrome::*;
use u__IMPDEF_boolean::*;
use Zeros::*;
use IsExternalAbort__1::*;
use HaveNV2Ext::*;
use IsSecondStage::*;
use EncodeLDFSC::*;
use AArch64_EncodeSyncErrorSyndrome::*;
use Bit::*;
use HaveFeatLS64::*;
use GetLoadStoreType::*;
use HaveRASExt::*;
use AArch64_PEErrorState::*;
use common::*;
pub fn AArch64_FaultSyndrome<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d_side: bool,
    fault: ProductType1d757adad216cdef,
    pavalid: bool,
) -> ProductType44ac89053e6d35a9 {
    #[derive(Default)]
    struct FunctionState {
        iss: u32,
        gs_9685: bool,
        ga_6871: bool,
        ga_6792: ProductType9878976b5bcce9c9,
        ga_6831: ProductType9878976b5bcce9c9,
        gs_9652: bool,
        gs_9701: bool,
        ga_6835: ProductType9878976b5bcce9c9,
        gs_9665: bool,
        ga_6897: bool,
        ga_6900: bool,
        iss2: u32,
        gs_9706: bool,
        ga_6810: ProductType9878976b5bcce9c9,
        gs_9662: bool,
        gs_9661: bool,
        gs_9707: bool,
        ga_6868: bool,
        ga_6818: ProductType9878976b5bcce9c9,
        gs_9702: bool,
        ga_6796: ProductType9878976b5bcce9c9,
        ga_6805: ProductType81b3b4c60b2f37ac,
        gs_9703: bool,
        gs_9643: bool,
        ga_6909: bool,
        gs_9699: bool,
        ga_6841: ProductType9878976b5bcce9c9,
        ga_6843: ProductType9878976b5bcce9c9,
        gs_9686: bool,
        gs_9668: bool,
        ga_6845: ProductType9878976b5bcce9c9,
        ga_6863: bool,
        gs_9666: bool,
        ga_6883: bool,
        gs_9663: bool,
        ga_6886: bool,
        gs_9667: bool,
        ga_6888: ProductType9878976b5bcce9c9,
        ga_6833: ProductType9878976b5bcce9c9,
        ga_6903: bool,
        gs_9664: bool,
        gs_9669: bool,
        ga_6857: ProductType9878976b5bcce9c9,
        ga_6880: bool,
        ga_6894: bool,
        gs_9700: bool,
        ga_6877: bool,
        d_side: bool,
        fault: ProductType1d757adad216cdef,
        pavalid: bool,
    }
    let fn_state = FunctionState {
        d_side,
        fault,
        pavalid,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
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
        // S s_0_10: cast reint s_0_9 -> u24
        let s_0_10: u32 = (s_0_9.value() as u32);
        // D s_0_11: write-var iss2 <= s_0_10
        fn_state.iss2 = s_0_10;
        // C s_0_12: const #() : ()
        let s_0_12: () = ();
        // S s_0_13: call HaveRASExt(s_0_12)
        let s_0_13: bool = HaveRASExt(state, tracer, s_0_12);
        // N s_0_14: branch s_0_13 b141 b1
        if s_0_13 {
            return block_141(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#9643 <= s_1_0
        fn_state.gs_9643 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_2_0: read-var gs#9643:u8
        let s_2_0: bool = fn_state.gs_9643;
        // N s_2_1: branch s_2_0 b140 b3
        if s_2_0 {
            return block_140(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_4_0: read-var d_side:u8
        let s_4_0: bool = fn_state.d_side;
        // N s_4_1: branch s_4_0 b31 b5
        if s_4_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_5_0: read-var fault.0:struct
        let s_5_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_5_1: write-var ga#6888 <= s_5_0
        fn_state.ga_6888 = s_5_0;
        // D s_5_2: read-var ga#6888.1:struct
        let s_5_2: u32 = fn_state.ga_6888._1;
        // C s_5_3: const #0u : u32
        let s_5_3: u32 = 0;
        // D s_5_4: cmp-eq s_5_2 s_5_3
        let s_5_4: bool = ((s_5_2) == (s_5_3));
        // N s_5_5: branch s_5_4 b30 b6
        if s_5_4 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#9652 <= s_6_0
        fn_state.gs_9652 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_7_0: read-var gs#9652:u8
        let s_7_0: bool = fn_state.gs_9652;
        // N s_7_1: branch s_7_0 b17 b8
        if s_7_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_10_0: read-var fault:struct
        let s_10_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_10_1: call IsExternalAbort__1(s_10_0)
        let s_10_1: bool = IsExternalAbort__1(state, tracer, s_10_0);
        // N s_10_2: branch s_10_1 b16 b11
        if s_10_1 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_12_0: read-var fault.14:struct
        let s_12_0: bool = fn_state.fault._14;
        // N s_12_1: branch s_12_0 b15 b13
        if s_12_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var ga#6909 <= s_13_0
        fn_state.ga_6909 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_14_0: read-var ga#6909:u8
        let s_14_0: bool = fn_state.ga_6909;
        // D s_14_1: call Bit(s_14_0)
        let s_14_1: bool = Bit(state, tracer, s_14_0);
        // C s_14_2: const #7s : i
        let s_14_2: i128 = 7;
        // D s_14_3: read-var iss:u25
        let s_14_3: u32 = fn_state.iss;
        // D s_14_4: cast zx s_14_3 -> bv
        let s_14_4: Bits = Bits::new(s_14_3 as u128, 25u16);
        // C s_14_5: const #1u : u64
        let s_14_5: u64 = 1;
        // D s_14_6: bit-insert s_14_4 s_14_4 s_14_2 s_14_5
        let s_14_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_14_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_14_4.length(),
            );
            (s_14_4 & mask) | (s_14_4 << s_14_2)
        };
        // D s_14_7: cast reint s_14_6 -> u25
        let s_14_7: u32 = (s_14_6.value() as u32);
        // D s_14_8: write-var iss <= s_14_7
        fn_state.iss = s_14_7;
        // D s_14_9: read-var fault.16:struct
        let s_14_9: u32 = fn_state.fault._16;
        // D s_14_10: read-var fault.9:struct
        let s_14_10: i128 = fn_state.fault._9;
        // D s_14_11: call EncodeLDFSC(s_14_9, s_14_10)
        let s_14_11: u8 = EncodeLDFSC(state, tracer, s_14_9, s_14_10);
        // C s_14_12: const #0s : i
        let s_14_12: i128 = 0;
        // D s_14_13: read-var iss:u25
        let s_14_13: u32 = fn_state.iss;
        // D s_14_14: cast zx s_14_13 -> bv
        let s_14_14: Bits = Bits::new(s_14_13 as u128, 25u16);
        // D s_14_15: cast zx s_14_11 -> bv
        let s_14_15: Bits = Bits::new(s_14_11 as u128, 6u16);
        // C s_14_16: const #5s : i
        let s_14_16: i128 = 5;
        // C s_14_17: const #1u : u64
        let s_14_17: u64 = 1;
        // C s_14_18: cast zx s_14_17 -> bv
        let s_14_18: Bits = Bits::new(s_14_17 as u128, 64u16);
        // C s_14_19: lsl s_14_18 s_14_16
        let s_14_19: Bits = s_14_18 << s_14_16;
        // C s_14_20: sub s_14_19 s_14_18
        let s_14_20: Bits = ((s_14_19) - (s_14_18));
        // D s_14_21: and s_14_15 s_14_20
        let s_14_21: Bits = ((s_14_15) & (s_14_20));
        // D s_14_22: lsl s_14_21 s_14_12
        let s_14_22: Bits = s_14_21 << s_14_12;
        // C s_14_23: lsl s_14_20 s_14_12
        let s_14_23: Bits = s_14_20 << s_14_12;
        // C s_14_24: cmpl s_14_23
        let s_14_24: Bits = !s_14_23;
        // D s_14_25: and s_14_14 s_14_24
        let s_14_25: Bits = ((s_14_14) & (s_14_24));
        // D s_14_26: or s_14_25 s_14_22
        let s_14_26: Bits = ((s_14_25) | (s_14_22));
        // D s_14_27: cast reint s_14_26 -> u25
        let s_14_27: u32 = (s_14_26.value() as u32);
        // D s_14_28: write-var iss <= s_14_27
        fn_state.iss = s_14_27;
        // D s_14_29: read-var iss:u25
        let s_14_29: u32 = fn_state.iss;
        // D s_14_30: read-var iss2:u24
        let s_14_30: u32 = fn_state.iss2;
        // D s_14_31: create-product struct = ["s_14_29", "s_14_30"]
        let s_14_31: ProductType44ac89053e6d35a9 = ProductType44ac89053e6d35a9 {
            _0: s_14_29,
            _1: s_14_30,
        };
        // N s_14_32: return s_14_31
        return s_14_31;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var ga#6909 <= s_15_0
        fn_state.ga_6909 = s_15_0;
        // N s_15_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_16_0: read-var fault.5:struct
        let s_16_0: bool = fn_state.fault._5;
        // D s_16_1: call Bit(s_16_0)
        let s_16_1: bool = Bit(state, tracer, s_16_0);
        // C s_16_2: const #9s : i
        let s_16_2: i128 = 9;
        // D s_16_3: read-var iss:u25
        let s_16_3: u32 = fn_state.iss;
        // D s_16_4: cast zx s_16_3 -> bv
        let s_16_4: Bits = Bits::new(s_16_3 as u128, 25u16);
        // C s_16_5: const #1u : u64
        let s_16_5: u64 = 1;
        // D s_16_6: bit-insert s_16_4 s_16_4 s_16_2 s_16_5
        let s_16_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_16_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_16_4.length(),
            );
            (s_16_4 & mask) | (s_16_4 << s_16_2)
        };
        // D s_16_7: cast reint s_16_6 -> u25
        let s_16_7: u32 = (s_16_6.value() as u32);
        // D s_16_8: write-var iss <= s_16_7
        fn_state.iss = s_16_7;
        // N s_16_9: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_17_0: read-var fault.3:struct
        let s_17_0: bool = fn_state.fault._3;
        // N s_17_1: branch s_17_0 b29 b18
        if s_17_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var ga#6894 <= s_18_0
        fn_state.ga_6894 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_19_0: read-var ga#6894:u8
        let s_19_0: bool = fn_state.ga_6894;
        // D s_19_1: call Bit(s_19_0)
        let s_19_1: bool = Bit(state, tracer, s_19_0);
        // C s_19_2: const #5s : i
        let s_19_2: i128 = 5;
        // D s_19_3: read-var iss2:u24
        let s_19_3: u32 = fn_state.iss2;
        // D s_19_4: cast zx s_19_3 -> bv
        let s_19_4: Bits = Bits::new(s_19_3 as u128, 24u16);
        // C s_19_5: const #1u : u64
        let s_19_5: u64 = 1;
        // D s_19_6: bit-insert s_19_4 s_19_4 s_19_2 s_19_5
        let s_19_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_19_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_19_4.length(),
            );
            (s_19_4 & mask) | (s_19_4 << s_19_2)
        };
        // D s_19_7: cast reint s_19_6 -> u24
        let s_19_7: u32 = (s_19_6.value() as u32);
        // D s_19_8: write-var iss2 <= s_19_7
        fn_state.iss2 = s_19_7;
        // D s_19_9: read-var fault.18:struct
        let s_19_9: bool = fn_state.fault._18;
        // N s_19_10: branch s_19_9 b28 b20
        if s_19_9 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var ga#6897 <= s_20_0
        fn_state.ga_6897 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_21_0: read-var ga#6897:u8
        let s_21_0: bool = fn_state.ga_6897;
        // D s_21_1: call Bit(s_21_0)
        let s_21_1: bool = Bit(state, tracer, s_21_0);
        // C s_21_2: const #21s : i
        let s_21_2: i128 = 21;
        // D s_21_3: read-var iss:u25
        let s_21_3: u32 = fn_state.iss;
        // D s_21_4: cast zx s_21_3 -> bv
        let s_21_4: Bits = Bits::new(s_21_3 as u128, 25u16);
        // C s_21_5: const #1u : u64
        let s_21_5: u64 = 1;
        // D s_21_6: bit-insert s_21_4 s_21_4 s_21_2 s_21_5
        let s_21_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_21_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_21_4.length(),
            );
            (s_21_4 & mask) | (s_21_4 << s_21_2)
        };
        // D s_21_7: cast reint s_21_6 -> u25
        let s_21_7: u32 = (s_21_6.value() as u32);
        // D s_21_8: write-var iss <= s_21_7
        fn_state.iss = s_21_7;
        // D s_21_9: read-var fault.1:struct
        let s_21_9: bool = fn_state.fault._1;
        // N s_21_10: branch s_21_9 b27 b22
        if s_21_9 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var ga#6900 <= s_22_0
        fn_state.ga_6900 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_23_0: read-var ga#6900:u8
        let s_23_0: bool = fn_state.ga_6900;
        // D s_23_1: call Bit(s_23_0)
        let s_23_1: bool = Bit(state, tracer, s_23_0);
        // C s_23_2: const #7s : i
        let s_23_2: i128 = 7;
        // D s_23_3: read-var iss2:u24
        let s_23_3: u32 = fn_state.iss2;
        // D s_23_4: cast zx s_23_3 -> bv
        let s_23_4: Bits = Bits::new(s_23_3 as u128, 24u16);
        // C s_23_5: const #1u : u64
        let s_23_5: u64 = 1;
        // D s_23_6: bit-insert s_23_4 s_23_4 s_23_2 s_23_5
        let s_23_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_23_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_23_4.length(),
            );
            (s_23_4 & mask) | (s_23_4 << s_23_2)
        };
        // D s_23_7: cast reint s_23_6 -> u24
        let s_23_7: u32 = (s_23_6.value() as u32);
        // D s_23_8: write-var iss2 <= s_23_7
        fn_state.iss2 = s_23_7;
        // D s_23_9: read-var fault.11:struct
        let s_23_9: bool = fn_state.fault._11;
        // N s_23_10: branch s_23_9 b26 b24
        if s_23_9 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var ga#6903 <= s_24_0
        fn_state.ga_6903 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_25_0: read-var ga#6903:u8
        let s_25_0: bool = fn_state.ga_6903;
        // D s_25_1: call Bit(s_25_0)
        let s_25_1: bool = Bit(state, tracer, s_25_0);
        // C s_25_2: const #6s : i
        let s_25_2: i128 = 6;
        // D s_25_3: read-var iss2:u24
        let s_25_3: u32 = fn_state.iss2;
        // D s_25_4: cast zx s_25_3 -> bv
        let s_25_4: Bits = Bits::new(s_25_3 as u128, 24u16);
        // C s_25_5: const #1u : u64
        let s_25_5: u64 = 1;
        // D s_25_6: bit-insert s_25_4 s_25_4 s_25_2 s_25_5
        let s_25_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_25_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_25_4.length(),
            );
            (s_25_4 & mask) | (s_25_4 << s_25_2)
        };
        // D s_25_7: cast reint s_25_6 -> u24
        let s_25_7: u32 = (s_25_6.value() as u32);
        // D s_25_8: write-var iss2 <= s_25_7
        fn_state.iss2 = s_25_7;
        // N s_25_9: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var ga#6903 <= s_26_0
        fn_state.ga_6903 = s_26_0;
        // N s_26_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var ga#6900 <= s_27_0
        fn_state.ga_6900 = s_27_0;
        // N s_27_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var ga#6897 <= s_28_0
        fn_state.ga_6897 = s_28_0;
        // N s_28_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var ga#6894 <= s_29_0
        fn_state.ga_6894 = s_29_0;
        // N s_29_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_30_0: read-var fault.16:struct
        let s_30_0: u32 = fn_state.fault._16;
        // C s_30_1: const #5u : u32
        let s_30_1: u32 = 5;
        // D s_30_2: cmp-eq s_30_0 s_30_1
        let s_30_2: bool = ((s_30_0) == (s_30_1));
        // D s_30_3: write-var gs#9652 <= s_30_2
        fn_state.gs_9652 = s_30_2;
        // N s_30_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_31_0: read-var fault.0:struct
        let s_31_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_31_1: write-var ga#6792 <= s_31_0
        fn_state.ga_6792 = s_31_0;
        // D s_31_2: read-var ga#6792.1:struct
        let s_31_2: u32 = fn_state.ga_6792._1;
        // C s_31_3: const #11u : u32
        let s_31_3: u32 = 11;
        // D s_31_4: cmp-eq s_31_2 s_31_3
        let s_31_4: bool = ((s_31_2) == (s_31_3));
        // N s_31_5: branch s_31_4 b139 b32
        if s_31_4 {
            return block_139(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // N s_32_0: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_33_0: const #() : ()
        let s_33_0: () = ();
        // S s_33_1: call HaveFeatLS64(s_33_0)
        let s_33_1: bool = HaveFeatLS64(state, tracer, s_33_0);
        // N s_33_2: branch s_33_1 b138 b34
        if s_33_1 {
            return block_138(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#9661 <= s_34_0
        fn_state.gs_9661 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_35_0: read-var gs#9661:u8
        let s_35_0: bool = fn_state.gs_9661;
        // N s_35_1: branch s_35_0 b128 b36
        if s_35_0 {
            return block_128(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_36_0: read-var fault:struct
        let s_36_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_36_1: call IsSecondStage(s_36_0)
        let s_36_1: bool = IsSecondStage(state, tracer, s_36_0);
        // N s_36_2: branch s_36_1 b127 b37
        if s_36_1 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var gs#9699 <= s_37_0
        fn_state.gs_9699 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_38_0: read-var gs#9699:u8
        let s_38_0: bool = fn_state.gs_9699;
        // N s_38_1: branch s_38_0 b117 b39
        if s_38_0 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#9703 <= s_39_0
        fn_state.gs_9703 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_40_0: read-var gs#9703:u8
        let s_40_0: bool = fn_state.gs_9703;
        // N s_40_1: branch s_40_0 b116 b41
        if s_40_0 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // N s_41_0: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call HaveNV2Ext(s_42_0)
        let s_42_1: bool = HaveNV2Ext(state, tracer, s_42_0);
        // N s_42_2: branch s_42_1 b115 b43
        if s_42_1 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#9662 <= s_43_0
        fn_state.gs_9662 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_44_0: read-var gs#9662:u8
        let s_44_0: bool = fn_state.gs_9662;
        // N s_44_1: branch s_44_0 b114 b45
        if s_44_0 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // N s_45_0: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_46_0: const #() : ()
        let s_46_0: () = ();
        // S s_46_1: call HaveFeatLS64(s_46_0)
        let s_46_1: bool = HaveFeatLS64(state, tracer, s_46_0);
        // N s_46_2: branch s_46_1 b107 b47
        if s_46_1 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_47_0: const #0u : u8
        let s_47_0: bool = false;
        // D s_47_1: write-var gs#9665 <= s_47_0
        fn_state.gs_9665 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_48_0: read-var gs#9665:u8
        let s_48_0: bool = fn_state.gs_9665;
        // N s_48_1: branch s_48_0 b106 b49
        if s_48_0 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // N s_49_0: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_50_0: read-var fault.0:struct
        let s_50_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_50_1: write-var ga#6831 <= s_50_0
        fn_state.ga_6831 = s_50_0;
        // D s_50_2: read-var ga#6831.1:struct
        let s_50_2: u32 = fn_state.ga_6831._1;
        // C s_50_3: const #6u : u32
        let s_50_3: u32 = 6;
        // D s_50_4: cmp-eq s_50_2 s_50_3
        let s_50_4: bool = ((s_50_2) == (s_50_3));
        // N s_50_5: branch s_50_4 b105 b51
        if s_50_4 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_51_0: read-var fault.0:struct
        let s_51_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_51_1: write-var ga#6833 <= s_51_0
        fn_state.ga_6833 = s_51_0;
        // D s_51_2: read-var ga#6833.1:struct
        let s_51_2: u32 = fn_state.ga_6833._1;
        // C s_51_3: const #5u : u32
        let s_51_3: u32 = 5;
        // D s_51_4: cmp-eq s_51_2 s_51_3
        let s_51_4: bool = ((s_51_2) == (s_51_3));
        // N s_51_5: branch s_51_4 b104 b52
        if s_51_4 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_52_0: read-var fault.0:struct
        let s_52_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_52_1: write-var ga#6835 <= s_52_0
        fn_state.ga_6835 = s_52_0;
        // D s_52_2: read-var ga#6835.1:struct
        let s_52_2: u32 = fn_state.ga_6835._1;
        // C s_52_3: const #8u : u32
        let s_52_3: u32 = 8;
        // D s_52_4: cmp-eq s_52_2 s_52_3
        let s_52_4: bool = ((s_52_2) == (s_52_3));
        // D s_52_5: write-var gs#9666 <= s_52_4
        fn_state.gs_9666 = s_52_4;
        // N s_52_6: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_53_0: read-var gs#9666:u8
        let s_53_0: bool = fn_state.gs_9666;
        // D s_53_1: write-var gs#9667 <= s_53_0
        fn_state.gs_9667 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_54_0: read-var gs#9667:u8
        let s_54_0: bool = fn_state.gs_9667;
        // N s_54_1: branch s_54_0 b103 b55
        if s_54_0 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // N s_55_0: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_56_0: read-var fault.0:struct
        let s_56_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_56_1: write-var ga#6841 <= s_56_0
        fn_state.ga_6841 = s_56_0;
        // D s_56_2: read-var ga#6841.1:struct
        let s_56_2: u32 = fn_state.ga_6841._1;
        // C s_56_3: const #6u : u32
        let s_56_3: u32 = 6;
        // D s_56_4: cmp-eq s_56_2 s_56_3
        let s_56_4: bool = ((s_56_2) == (s_56_3));
        // N s_56_5: branch s_56_4 b102 b57
        if s_56_4 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_57_0: read-var fault.0:struct
        let s_57_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_57_1: write-var ga#6843 <= s_57_0
        fn_state.ga_6843 = s_57_0;
        // D s_57_2: read-var ga#6843.1:struct
        let s_57_2: u32 = fn_state.ga_6843._1;
        // C s_57_3: const #5u : u32
        let s_57_3: u32 = 5;
        // D s_57_4: cmp-eq s_57_2 s_57_3
        let s_57_4: bool = ((s_57_2) == (s_57_3));
        // N s_57_5: branch s_57_4 b101 b58
        if s_57_4 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_58_0: read-var fault.0:struct
        let s_58_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_58_1: write-var ga#6845 <= s_58_0
        fn_state.ga_6845 = s_58_0;
        // D s_58_2: read-var ga#6845.1:struct
        let s_58_2: u32 = fn_state.ga_6845._1;
        // C s_58_3: const #8u : u32
        let s_58_3: u32 = 8;
        // D s_58_4: cmp-eq s_58_2 s_58_3
        let s_58_4: bool = ((s_58_2) == (s_58_3));
        // D s_58_5: write-var gs#9668 <= s_58_4
        fn_state.gs_9668 = s_58_4;
        // N s_58_6: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_59_0: read-var gs#9668:u8
        let s_59_0: bool = fn_state.gs_9668;
        // D s_59_1: write-var gs#9669 <= s_59_0
        fn_state.gs_9669 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_60_0: read-var gs#9669:u8
        let s_60_0: bool = fn_state.gs_9669;
        // N s_60_1: branch s_60_0 b100 b61
        if s_60_0 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_61_0: read-var fault.16:struct
        let s_61_0: u32 = fn_state.fault._16;
        // C s_61_1: const #20u : u32
        let s_61_1: u32 = 20;
        // D s_61_2: cmp-eq s_61_0 s_61_1
        let s_61_2: bool = ((s_61_0) == (s_61_1));
        // N s_61_3: branch s_61_2 b99 b62
        if s_61_2 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_62_0: read-var fault.16:struct
        let s_62_0: u32 = fn_state.fault._16;
        // C s_62_1: const #22u : u32
        let s_62_1: u32 = 22;
        // D s_62_2: cmp-eq s_62_0 s_62_1
        let s_62_2: bool = ((s_62_0) == (s_62_1));
        // D s_62_3: write-var gs#9685 <= s_62_2
        fn_state.gs_9685 = s_62_2;
        // N s_62_4: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_63_0: read-var gs#9685:u8
        let s_63_0: bool = fn_state.gs_9685;
        // N s_63_1: branch s_63_0 b98 b64
        if s_63_0 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_64_0: read-var fault.0:struct
        let s_64_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_64_1: write-var ga#6857 <= s_64_0
        fn_state.ga_6857 = s_64_0;
        // D s_64_2: read-var ga#6857.4:struct
        let s_64_2: bool = fn_state.ga_6857._4;
        // N s_64_3: branch s_64_2 b97 b65
        if s_64_2 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_65_0: const #0u : u8
        let s_65_0: bool = false;
        // D s_65_1: write-var gs#9686 <= s_65_0
        fn_state.gs_9686 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_66_0: read-var gs#9686:u8
        let s_66_0: bool = fn_state.gs_9686;
        // N s_66_1: branch s_66_0 b96 b67
        if s_66_0 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_67_0: read-var fault.19:struct
        let s_67_0: bool = fn_state.fault._19;
        // N s_67_1: branch s_67_0 b95 b68
        if s_67_0 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_68_0: const #0u : u8
        let s_68_0: bool = false;
        // D s_68_1: write-var ga#6863 <= s_68_0
        fn_state.ga_6863 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_69_0: read-var ga#6863:u8
        let s_69_0: bool = fn_state.ga_6863;
        // D s_69_1: call Bit(s_69_0)
        let s_69_1: bool = Bit(state, tracer, s_69_0);
        // C s_69_2: const #6s : i
        let s_69_2: i128 = 6;
        // D s_69_3: read-var iss:u25
        let s_69_3: u32 = fn_state.iss;
        // D s_69_4: cast zx s_69_3 -> bv
        let s_69_4: Bits = Bits::new(s_69_3 as u128, 25u16);
        // C s_69_5: const #1u : u64
        let s_69_5: u64 = 1;
        // D s_69_6: bit-insert s_69_4 s_69_4 s_69_2 s_69_5
        let s_69_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_69_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_69_4.length(),
            );
            (s_69_4 & mask) | (s_69_4 << s_69_2)
        };
        // D s_69_7: cast reint s_69_6 -> u25
        let s_69_7: u32 = (s_69_6.value() as u32);
        // D s_69_8: write-var iss <= s_69_7
        fn_state.iss = s_69_7;
        // N s_69_9: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_70_0: read-var fault.16:struct
        let s_70_0: u32 = fn_state.fault._16;
        // C s_70_1: const #5u : u32
        let s_70_1: u32 = 5;
        // D s_70_2: cmp-eq s_70_0 s_70_1
        let s_70_2: bool = ((s_70_0) == (s_70_1));
        // N s_70_3: branch s_70_2 b73 b71
        if s_70_2 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // N s_71_0: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // N s_72_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_73_0: read-var fault.3:struct
        let s_73_0: bool = fn_state.fault._3;
        // N s_73_1: branch s_73_0 b94 b74
        if s_73_0 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_74_0: const #0u : u8
        let s_74_0: bool = false;
        // D s_74_1: write-var ga#6868 <= s_74_0
        fn_state.ga_6868 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_75_0: read-var ga#6868:u8
        let s_75_0: bool = fn_state.ga_6868;
        // D s_75_1: call Bit(s_75_0)
        let s_75_1: bool = Bit(state, tracer, s_75_0);
        // C s_75_2: const #5s : i
        let s_75_2: i128 = 5;
        // D s_75_3: read-var iss2:u24
        let s_75_3: u32 = fn_state.iss2;
        // D s_75_4: cast zx s_75_3 -> bv
        let s_75_4: Bits = Bits::new(s_75_3 as u128, 24u16);
        // C s_75_5: const #1u : u64
        let s_75_5: u64 = 1;
        // D s_75_6: bit-insert s_75_4 s_75_4 s_75_2 s_75_5
        let s_75_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_75_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_75_4.length(),
            );
            (s_75_4 & mask) | (s_75_4 << s_75_2)
        };
        // D s_75_7: cast reint s_75_6 -> u24
        let s_75_7: u32 = (s_75_6.value() as u32);
        // D s_75_8: write-var iss2 <= s_75_7
        fn_state.iss2 = s_75_7;
        // D s_75_9: read-var fault.11:struct
        let s_75_9: bool = fn_state.fault._11;
        // N s_75_10: branch s_75_9 b93 b76
        if s_75_9 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_76_0: const #0u : u8
        let s_76_0: bool = false;
        // D s_76_1: write-var ga#6871 <= s_76_0
        fn_state.ga_6871 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_77_0: read-var ga#6871:u8
        let s_77_0: bool = fn_state.ga_6871;
        // D s_77_1: call Bit(s_77_0)
        let s_77_1: bool = Bit(state, tracer, s_77_0);
        // C s_77_2: const #6s : i
        let s_77_2: i128 = 6;
        // D s_77_3: read-var iss2:u24
        let s_77_3: u32 = fn_state.iss2;
        // D s_77_4: cast zx s_77_3 -> bv
        let s_77_4: Bits = Bits::new(s_77_3 as u128, 24u16);
        // C s_77_5: const #1u : u64
        let s_77_5: u64 = 1;
        // D s_77_6: bit-insert s_77_4 s_77_4 s_77_2 s_77_5
        let s_77_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_77_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_77_4.length(),
            );
            (s_77_4 & mask) | (s_77_4 << s_77_2)
        };
        // D s_77_7: cast reint s_77_6 -> u24
        let s_77_7: u32 = (s_77_6.value() as u32);
        // D s_77_8: write-var iss2 <= s_77_7
        fn_state.iss2 = s_77_7;
        // C s_77_9: const #24s : i
        let s_77_9: i128 = 24;
        // D s_77_10: read-var iss:u25
        let s_77_10: u32 = fn_state.iss;
        // D s_77_11: cast zx s_77_10 -> bv
        let s_77_11: Bits = Bits::new(s_77_10 as u128, 25u16);
        // C s_77_12: const #1u : u64
        let s_77_12: u64 = 1;
        // D s_77_13: bit-extract s_77_11 s_77_9 s_77_12
        let s_77_13: Bits = (Bits::new(
            ((s_77_11) >> (s_77_9)).value(),
            u16::try_from(s_77_12).unwrap(),
        ));
        // D s_77_14: cast reint s_77_13 -> u8
        let s_77_14: bool = ((s_77_13.value()) != 0);
        // C s_77_15: const #0s : i
        let s_77_15: i128 = 0;
        // C s_77_16: const #0u : u64
        let s_77_16: u64 = 0;
        // D s_77_17: cast zx s_77_14 -> u64
        let s_77_17: u64 = (s_77_14 as u64);
        // C s_77_18: const #1u : u64
        let s_77_18: u64 = 1;
        // D s_77_19: and s_77_17 s_77_18
        let s_77_19: u64 = ((s_77_17) & (s_77_18));
        // D s_77_20: cmp-eq s_77_19 s_77_18
        let s_77_20: bool = ((s_77_19) == (s_77_18));
        // D s_77_21: lsl s_77_17 s_77_15
        let s_77_21: u64 = s_77_17 << s_77_15;
        // D s_77_22: or s_77_16 s_77_21
        let s_77_22: u64 = ((s_77_16) | (s_77_21));
        // D s_77_23: cmpl s_77_21
        let s_77_23: u64 = !s_77_21;
        // D s_77_24: and s_77_16 s_77_23
        let s_77_24: u64 = ((s_77_16) & (s_77_23));
        // D s_77_25: select s_77_20 s_77_22 s_77_24
        let s_77_25: u64 = if s_77_20 { s_77_22 } else { s_77_24 };
        // D s_77_26: cast trunc s_77_25 -> u8
        let s_77_26: bool = ((s_77_25) != 0);
        // D s_77_27: cast zx s_77_26 -> bv
        let s_77_27: Bits = Bits::new(s_77_26 as u128, 1u16);
        // C s_77_28: const #0u : u8
        let s_77_28: bool = false;
        // C s_77_29: cast zx s_77_28 -> bv
        let s_77_29: Bits = Bits::new(s_77_28 as u128, 1u16);
        // D s_77_30: cmp-eq s_77_27 s_77_29
        let s_77_30: bool = ((s_77_27) == (s_77_29));
        // N s_77_31: branch s_77_30 b89 b78
        if s_77_30 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // N s_78_0: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_79_0: read-var fault.1:struct
        let s_79_0: bool = fn_state.fault._1;
        // N s_79_1: branch s_79_0 b88 b80
        if s_79_0 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_80_0: const #0u : u8
        let s_80_0: bool = false;
        // D s_80_1: write-var ga#6880 <= s_80_0
        fn_state.ga_6880 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_81_0: read-var ga#6880:u8
        let s_81_0: bool = fn_state.ga_6880;
        // D s_81_1: call Bit(s_81_0)
        let s_81_1: bool = Bit(state, tracer, s_81_0);
        // C s_81_2: const #7s : i
        let s_81_2: i128 = 7;
        // D s_81_3: read-var iss2:u24
        let s_81_3: u32 = fn_state.iss2;
        // D s_81_4: cast zx s_81_3 -> bv
        let s_81_4: Bits = Bits::new(s_81_3 as u128, 24u16);
        // C s_81_5: const #1u : u64
        let s_81_5: u64 = 1;
        // D s_81_6: bit-insert s_81_4 s_81_4 s_81_2 s_81_5
        let s_81_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_81_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_81_4.length(),
            );
            (s_81_4 & mask) | (s_81_4 << s_81_2)
        };
        // D s_81_7: cast reint s_81_6 -> u24
        let s_81_7: u32 = (s_81_6.value() as u32);
        // D s_81_8: write-var iss2 <= s_81_7
        fn_state.iss2 = s_81_7;
        // D s_81_9: read-var fault.17:struct
        let s_81_9: bool = fn_state.fault._17;
        // N s_81_10: branch s_81_9 b87 b82
        if s_81_9 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_82_0: const #0u : u8
        let s_82_0: bool = false;
        // D s_82_1: write-var ga#6883 <= s_82_0
        fn_state.ga_6883 = s_82_0;
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_83_0: read-var ga#6883:u8
        let s_83_0: bool = fn_state.ga_6883;
        // D s_83_1: call Bit(s_83_0)
        let s_83_1: bool = Bit(state, tracer, s_83_0);
        // C s_83_2: const #9s : i
        let s_83_2: i128 = 9;
        // D s_83_3: read-var iss2:u24
        let s_83_3: u32 = fn_state.iss2;
        // D s_83_4: cast zx s_83_3 -> bv
        let s_83_4: Bits = Bits::new(s_83_3 as u128, 24u16);
        // C s_83_5: const #1u : u64
        let s_83_5: u64 = 1;
        // D s_83_6: bit-insert s_83_4 s_83_4 s_83_2 s_83_5
        let s_83_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_83_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_83_4.length(),
            );
            (s_83_4 & mask) | (s_83_4 << s_83_2)
        };
        // D s_83_7: cast reint s_83_6 -> u24
        let s_83_7: u32 = (s_83_6.value() as u32);
        // D s_83_8: write-var iss2 <= s_83_7
        fn_state.iss2 = s_83_7;
        // D s_83_9: read-var fault.13:struct
        let s_83_9: bool = fn_state.fault._13;
        // N s_83_10: branch s_83_9 b86 b84
        if s_83_9 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_84_0: const #0u : u8
        let s_84_0: bool = false;
        // D s_84_1: write-var ga#6886 <= s_84_0
        fn_state.ga_6886 = s_84_0;
        // N s_84_2: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_85_0: read-var ga#6886:u8
        let s_85_0: bool = fn_state.ga_6886;
        // D s_85_1: call Bit(s_85_0)
        let s_85_1: bool = Bit(state, tracer, s_85_0);
        // C s_85_2: const #10s : i
        let s_85_2: i128 = 10;
        // D s_85_3: read-var iss2:u24
        let s_85_3: u32 = fn_state.iss2;
        // D s_85_4: cast zx s_85_3 -> bv
        let s_85_4: Bits = Bits::new(s_85_3 as u128, 24u16);
        // C s_85_5: const #1u : u64
        let s_85_5: u64 = 1;
        // D s_85_6: bit-insert s_85_4 s_85_4 s_85_2 s_85_5
        let s_85_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_85_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_85_4.length(),
            );
            (s_85_4 & mask) | (s_85_4 << s_85_2)
        };
        // D s_85_7: cast reint s_85_6 -> u24
        let s_85_7: u32 = (s_85_6.value() as u32);
        // D s_85_8: write-var iss2 <= s_85_7
        fn_state.iss2 = s_85_7;
        // N s_85_9: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_86_0: const #1u : u8
        let s_86_0: bool = true;
        // D s_86_1: write-var ga#6886 <= s_86_0
        fn_state.ga_6886 = s_86_0;
        // N s_86_2: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_87_0: const #1u : u8
        let s_87_0: bool = true;
        // D s_87_1: write-var ga#6883 <= s_87_0
        fn_state.ga_6883 = s_87_0;
        // N s_87_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_88_0: const #1u : u8
        let s_88_0: bool = true;
        // D s_88_1: write-var ga#6880 <= s_88_0
        fn_state.ga_6880 = s_88_0;
        // N s_88_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_89_0: read-var fault.18:struct
        let s_89_0: bool = fn_state.fault._18;
        // N s_89_1: branch s_89_0 b92 b90
        if s_89_0 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_90_0: const #0u : u8
        let s_90_0: bool = false;
        // D s_90_1: write-var ga#6877 <= s_90_0
        fn_state.ga_6877 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_91_0: read-var ga#6877:u8
        let s_91_0: bool = fn_state.ga_6877;
        // D s_91_1: call Bit(s_91_0)
        let s_91_1: bool = Bit(state, tracer, s_91_0);
        // C s_91_2: const #21s : i
        let s_91_2: i128 = 21;
        // D s_91_3: read-var iss:u25
        let s_91_3: u32 = fn_state.iss;
        // D s_91_4: cast zx s_91_3 -> bv
        let s_91_4: Bits = Bits::new(s_91_3 as u128, 25u16);
        // C s_91_5: const #1u : u64
        let s_91_5: u64 = 1;
        // D s_91_6: bit-insert s_91_4 s_91_4 s_91_2 s_91_5
        let s_91_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_91_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_91_4.length(),
            );
            (s_91_4 & mask) | (s_91_4 << s_91_2)
        };
        // D s_91_7: cast reint s_91_6 -> u25
        let s_91_7: u32 = (s_91_6.value() as u32);
        // D s_91_8: write-var iss <= s_91_7
        fn_state.iss = s_91_7;
        // N s_91_9: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_92_0: const #1u : u8
        let s_92_0: bool = true;
        // D s_92_1: write-var ga#6877 <= s_92_0
        fn_state.ga_6877 = s_92_0;
        // N s_92_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_93_0: const #1u : u8
        let s_93_0: bool = true;
        // D s_93_1: write-var ga#6871 <= s_93_0
        fn_state.ga_6871 = s_93_0;
        // N s_93_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_94_0: const #1u : u8
        let s_94_0: bool = true;
        // D s_94_1: write-var ga#6868 <= s_94_0
        fn_state.ga_6868 = s_94_0;
        // N s_94_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_95_0: const #1u : u8
        let s_95_0: bool = true;
        // D s_95_1: write-var ga#6863 <= s_95_0
        fn_state.ga_6863 = s_95_0;
        // N s_95_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_96_0: const #() : ()
        let s_96_0: () = ();
        // S s_96_1: call __UNKNOWN_bit(s_96_0)
        let s_96_1: bool = u__UNKNOWN_bit(state, tracer, s_96_0);
        // S s_96_2: call Bit(s_96_1)
        let s_96_2: bool = Bit(state, tracer, s_96_1);
        // C s_96_3: const #6s : i
        let s_96_3: i128 = 6;
        // D s_96_4: read-var iss:u25
        let s_96_4: u32 = fn_state.iss;
        // D s_96_5: cast zx s_96_4 -> bv
        let s_96_5: Bits = Bits::new(s_96_4 as u128, 25u16);
        // C s_96_6: const #1u : u64
        let s_96_6: u64 = 1;
        // D s_96_7: bit-insert s_96_5 s_96_5 s_96_3 s_96_6
        let s_96_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_96_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_96_5.length(),
            );
            (s_96_5 & mask) | (s_96_5 << s_96_3)
        };
        // D s_96_8: cast reint s_96_7 -> u25
        let s_96_8: u32 = (s_96_7.value() as u32);
        // D s_96_9: write-var iss <= s_96_8
        fn_state.iss = s_96_8;
        // N s_96_10: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_97_0: read-var fault:struct
        let s_97_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_97_1: call IsExternalAbort__1(s_97_0)
        let s_97_1: bool = IsExternalAbort__1(state, tracer, s_97_0);
        // D s_97_2: write-var gs#9686 <= s_97_1
        fn_state.gs_9686 = s_97_1;
        // N s_97_3: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_98_0: const #() : ()
        let s_98_0: () = ();
        // S s_98_1: call __UNKNOWN_bit(s_98_0)
        let s_98_1: bool = u__UNKNOWN_bit(state, tracer, s_98_0);
        // S s_98_2: call Bit(s_98_1)
        let s_98_2: bool = Bit(state, tracer, s_98_1);
        // C s_98_3: const #6s : i
        let s_98_3: i128 = 6;
        // D s_98_4: read-var iss:u25
        let s_98_4: u32 = fn_state.iss;
        // D s_98_5: cast zx s_98_4 -> bv
        let s_98_5: Bits = Bits::new(s_98_4 as u128, 25u16);
        // C s_98_6: const #1u : u64
        let s_98_6: u64 = 1;
        // D s_98_7: bit-insert s_98_5 s_98_5 s_98_3 s_98_6
        let s_98_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_98_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_98_5.length(),
            );
            (s_98_5 & mask) | (s_98_5 << s_98_3)
        };
        // D s_98_8: cast reint s_98_7 -> u25
        let s_98_8: u32 = (s_98_7.value() as u32);
        // D s_98_9: write-var iss <= s_98_8
        fn_state.iss = s_98_8;
        // N s_98_10: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_99_0: const #1u : u8
        let s_99_0: bool = true;
        // D s_99_1: write-var gs#9685 <= s_99_0
        fn_state.gs_9685 = s_99_0;
        // N s_99_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_100_0: const #1u : u8
        let s_100_0: bool = true;
        // S s_100_1: call Bit(s_100_0)
        let s_100_1: bool = Bit(state, tracer, s_100_0);
        // C s_100_2: const #6s : i
        let s_100_2: i128 = 6;
        // D s_100_3: read-var iss:u25
        let s_100_3: u32 = fn_state.iss;
        // D s_100_4: cast zx s_100_3 -> bv
        let s_100_4: Bits = Bits::new(s_100_3 as u128, 25u16);
        // C s_100_5: const #1u : u64
        let s_100_5: u64 = 1;
        // D s_100_6: bit-insert s_100_4 s_100_4 s_100_2 s_100_5
        let s_100_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_100_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_100_4.length(),
            );
            (s_100_4 & mask) | (s_100_4 << s_100_2)
        };
        // D s_100_7: cast reint s_100_6 -> u25
        let s_100_7: u32 = (s_100_6.value() as u32);
        // D s_100_8: write-var iss <= s_100_7
        fn_state.iss = s_100_7;
        // N s_100_9: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_101_0: const #1u : u8
        let s_101_0: bool = true;
        // D s_101_1: write-var gs#9668 <= s_101_0
        fn_state.gs_9668 = s_101_0;
        // N s_101_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_102_0: const #1u : u8
        let s_102_0: bool = true;
        // D s_102_1: write-var gs#9669 <= s_102_0
        fn_state.gs_9669 = s_102_0;
        // N s_102_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_103_0: const #1u : u8
        let s_103_0: bool = true;
        // S s_103_1: call Bit(s_103_0)
        let s_103_1: bool = Bit(state, tracer, s_103_0);
        // C s_103_2: const #8s : i
        let s_103_2: i128 = 8;
        // D s_103_3: read-var iss:u25
        let s_103_3: u32 = fn_state.iss;
        // D s_103_4: cast zx s_103_3 -> bv
        let s_103_4: Bits = Bits::new(s_103_3 as u128, 25u16);
        // C s_103_5: const #1u : u64
        let s_103_5: u64 = 1;
        // D s_103_6: bit-insert s_103_4 s_103_4 s_103_2 s_103_5
        let s_103_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_103_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_103_4.length(),
            );
            (s_103_4 & mask) | (s_103_4 << s_103_2)
        };
        // D s_103_7: cast reint s_103_6 -> u25
        let s_103_7: u32 = (s_103_6.value() as u32);
        // D s_103_8: write-var iss <= s_103_7
        fn_state.iss = s_103_7;
        // N s_103_9: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_104_0: const #1u : u8
        let s_104_0: bool = true;
        // D s_104_1: write-var gs#9666 <= s_104_0
        fn_state.gs_9666 = s_104_0;
        // N s_104_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_105_0: const #1u : u8
        let s_105_0: bool = true;
        // D s_105_1: write-var gs#9667 <= s_105_0
        fn_state.gs_9667 = s_105_0;
        // N s_105_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_106_0: const #() : ()
        let s_106_0: () = ();
        // S s_106_1: call GetLoadStoreType(s_106_0)
        let s_106_1: u8 = GetLoadStoreType(state, tracer, s_106_0);
        // C s_106_2: const #11s : i
        let s_106_2: i128 = 11;
        // D s_106_3: read-var iss:u25
        let s_106_3: u32 = fn_state.iss;
        // D s_106_4: cast zx s_106_3 -> bv
        let s_106_4: Bits = Bits::new(s_106_3 as u128, 25u16);
        // S s_106_5: cast zx s_106_1 -> bv
        let s_106_5: Bits = Bits::new(s_106_1 as u128, 2u16);
        // C s_106_6: const #1s : i
        let s_106_6: i128 = 1;
        // C s_106_7: const #1u : u64
        let s_106_7: u64 = 1;
        // C s_106_8: cast zx s_106_7 -> bv
        let s_106_8: Bits = Bits::new(s_106_7 as u128, 64u16);
        // C s_106_9: lsl s_106_8 s_106_6
        let s_106_9: Bits = s_106_8 << s_106_6;
        // C s_106_10: sub s_106_9 s_106_8
        let s_106_10: Bits = ((s_106_9) - (s_106_8));
        // S s_106_11: and s_106_5 s_106_10
        let s_106_11: Bits = ((s_106_5) & (s_106_10));
        // S s_106_12: lsl s_106_11 s_106_2
        let s_106_12: Bits = s_106_11 << s_106_2;
        // C s_106_13: lsl s_106_10 s_106_2
        let s_106_13: Bits = s_106_10 << s_106_2;
        // C s_106_14: cmpl s_106_13
        let s_106_14: Bits = !s_106_13;
        // D s_106_15: and s_106_4 s_106_14
        let s_106_15: Bits = ((s_106_4) & (s_106_14));
        // D s_106_16: or s_106_15 s_106_12
        let s_106_16: Bits = ((s_106_15) | (s_106_12));
        // D s_106_17: cast reint s_106_16 -> u25
        let s_106_17: u32 = (s_106_16.value() as u32);
        // D s_106_18: write-var iss <= s_106_17
        fn_state.iss = s_106_17;
        // N s_106_19: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_107_0: read-var fault.16:struct
        let s_107_0: u32 = fn_state.fault._16;
        // C s_107_1: const #1u : u32
        let s_107_1: u32 = 1;
        // D s_107_2: cmp-eq s_107_0 s_107_1
        let s_107_2: bool = ((s_107_0) == (s_107_1));
        // N s_107_3: branch s_107_2 b113 b108
        if s_107_2 {
            return block_113(state, tracer, fn_state);
        } else {
            return block_108(state, tracer, fn_state);
        };
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_108_0: read-var fault.16:struct
        let s_108_0: u32 = fn_state.fault._16;
        // C s_108_1: const #6u : u32
        let s_108_1: u32 = 6;
        // D s_108_2: cmp-eq s_108_0 s_108_1
        let s_108_2: bool = ((s_108_0) == (s_108_1));
        // N s_108_3: branch s_108_2 b112 b109
        if s_108_2 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_109_0: read-var fault.16:struct
        let s_109_0: u32 = fn_state.fault._16;
        // C s_109_1: const #5u : u32
        let s_109_1: u32 = 5;
        // D s_109_2: cmp-eq s_109_0 s_109_1
        let s_109_2: bool = ((s_109_0) == (s_109_1));
        // D s_109_3: write-var gs#9663 <= s_109_2
        fn_state.gs_9663 = s_109_2;
        // N s_109_4: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_110_0: read-var gs#9663:u8
        let s_110_0: bool = fn_state.gs_9663;
        // D s_110_1: write-var gs#9664 <= s_110_0
        fn_state.gs_9664 = s_110_0;
        // N s_110_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_111_0: read-var gs#9664:u8
        let s_111_0: bool = fn_state.gs_9664;
        // D s_111_1: write-var gs#9665 <= s_111_0
        fn_state.gs_9665 = s_111_0;
        // N s_111_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_112_0: const #1u : u8
        let s_112_0: bool = true;
        // D s_112_1: write-var gs#9663 <= s_112_0
        fn_state.gs_9663 = s_112_0;
        // N s_112_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_113_0: const #1u : u8
        let s_113_0: bool = true;
        // D s_113_1: write-var gs#9664 <= s_113_0
        fn_state.gs_9664 = s_113_0;
        // N s_113_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_114_0: const #1u : u8
        let s_114_0: bool = true;
        // S s_114_1: call Bit(s_114_0)
        let s_114_1: bool = Bit(state, tracer, s_114_0);
        // C s_114_2: const #13s : i
        let s_114_2: i128 = 13;
        // D s_114_3: read-var iss:u25
        let s_114_3: u32 = fn_state.iss;
        // D s_114_4: cast zx s_114_3 -> bv
        let s_114_4: Bits = Bits::new(s_114_3 as u128, 25u16);
        // C s_114_5: const #1u : u64
        let s_114_5: u64 = 1;
        // D s_114_6: bit-insert s_114_4 s_114_4 s_114_2 s_114_5
        let s_114_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_114_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_114_4.length(),
            );
            (s_114_4 & mask) | (s_114_4 << s_114_2)
        };
        // D s_114_7: cast reint s_114_6 -> u25
        let s_114_7: u32 = (s_114_6.value() as u32);
        // D s_114_8: write-var iss <= s_114_7
        fn_state.iss = s_114_7;
        // N s_114_9: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_115_0: read-var fault.0:struct
        let s_115_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_115_1: write-var ga#6818 <= s_115_0
        fn_state.ga_6818 = s_115_0;
        // D s_115_2: read-var ga#6818.1:struct
        let s_115_2: u32 = fn_state.ga_6818._1;
        // C s_115_3: const #9u : u32
        let s_115_3: u32 = 9;
        // D s_115_4: cmp-eq s_115_2 s_115_3
        let s_115_4: bool = ((s_115_2) == (s_115_3));
        // D s_115_5: write-var gs#9662 <= s_115_4
        fn_state.gs_9662 = s_115_4;
        // N s_115_6: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_116_0: const #() : ()
        let s_116_0: () = ();
        // S s_116_1: call LSInstructionSyndrome(s_116_0)
        let s_116_1: u16 = LSInstructionSyndrome(state, tracer, s_116_0);
        // C s_116_2: const #14s : i
        let s_116_2: i128 = 14;
        // D s_116_3: read-var iss:u25
        let s_116_3: u32 = fn_state.iss;
        // D s_116_4: cast zx s_116_3 -> bv
        let s_116_4: Bits = Bits::new(s_116_3 as u128, 25u16);
        // S s_116_5: cast zx s_116_1 -> bv
        let s_116_5: Bits = Bits::new(s_116_1 as u128, 11u16);
        // C s_116_6: const #10s : i
        let s_116_6: i128 = 10;
        // C s_116_7: const #1u : u64
        let s_116_7: u64 = 1;
        // C s_116_8: cast zx s_116_7 -> bv
        let s_116_8: Bits = Bits::new(s_116_7 as u128, 64u16);
        // C s_116_9: lsl s_116_8 s_116_6
        let s_116_9: Bits = s_116_8 << s_116_6;
        // C s_116_10: sub s_116_9 s_116_8
        let s_116_10: Bits = ((s_116_9) - (s_116_8));
        // S s_116_11: and s_116_5 s_116_10
        let s_116_11: Bits = ((s_116_5) & (s_116_10));
        // S s_116_12: lsl s_116_11 s_116_2
        let s_116_12: Bits = s_116_11 << s_116_2;
        // C s_116_13: lsl s_116_10 s_116_2
        let s_116_13: Bits = s_116_10 << s_116_2;
        // C s_116_14: cmpl s_116_13
        let s_116_14: Bits = !s_116_13;
        // D s_116_15: and s_116_4 s_116_14
        let s_116_15: Bits = ((s_116_4) & (s_116_14));
        // D s_116_16: or s_116_15 s_116_12
        let s_116_16: Bits = ((s_116_15) | (s_116_12));
        // D s_116_17: cast reint s_116_16 -> u25
        let s_116_17: u32 = (s_116_16.value() as u32);
        // D s_116_18: write-var iss <= s_116_17
        fn_state.iss = s_116_17;
        // N s_116_19: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_117_0: read-var fault:struct
        let s_117_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_117_1: call IsExternalSyncAbort__1(s_117_0)
        let s_117_1: bool = IsExternalSyncAbort__1(state, tracer, s_117_0);
        // D s_117_2: not s_117_1
        let s_117_2: bool = !s_117_1;
        // N s_117_3: branch s_117_2 b126 b118
        if s_117_2 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_118(state, tracer, fn_state);
        };
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_118_0: const #() : ()
        let s_118_0: () = ();
        // S s_118_1: call HaveRASExt(s_118_0)
        let s_118_1: bool = HaveRASExt(state, tracer, s_118_0);
        // S s_118_2: not s_118_1
        let s_118_2: bool = !s_118_1;
        // N s_118_3: branch s_118_2 b125 b119
        if s_118_2 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_119_0: const #0u : u8
        let s_119_0: bool = false;
        // D s_119_1: write-var gs#9700 <= s_119_0
        fn_state.gs_9700 = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_120_0: read-var gs#9700:u8
        let s_120_0: bool = fn_state.gs_9700;
        // N s_120_1: branch s_120_0 b124 b121
        if s_120_0 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_121_0: const #0u : u8
        let s_121_0: bool = false;
        // D s_121_1: write-var gs#9701 <= s_121_0
        fn_state.gs_9701 = s_121_0;
        // N s_121_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_122_0: read-var gs#9701:u8
        let s_122_0: bool = fn_state.gs_9701;
        // D s_122_1: write-var gs#9702 <= s_122_0
        fn_state.gs_9702 = s_122_0;
        // N s_122_2: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_123_0: read-var gs#9702:u8
        let s_123_0: bool = fn_state.gs_9702;
        // D s_123_1: write-var gs#9703 <= s_123_0
        fn_state.gs_9703 = s_123_0;
        // N s_123_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_124_0: const #"ISV on second stage translation table walk" : str
        let s_124_0: &'static str = "ISV on second stage translation table walk";
        // S s_124_1: call __IMPDEF_boolean(s_124_0)
        let s_124_1: bool = u__IMPDEF_boolean(state, tracer, s_124_0);
        // D s_124_2: write-var gs#9701 <= s_124_1
        fn_state.gs_9701 = s_124_1;
        // N s_124_3: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_125_0: read-var fault.0:struct
        let s_125_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_125_1: write-var ga#6810 <= s_125_0
        fn_state.ga_6810 = s_125_0;
        // D s_125_2: read-var ga#6810.1:struct
        let s_125_2: u32 = fn_state.ga_6810._1;
        // C s_125_3: const #13u : u32
        let s_125_3: u32 = 13;
        // D s_125_4: cmp-eq s_125_2 s_125_3
        let s_125_4: bool = ((s_125_2) == (s_125_3));
        // D s_125_5: write-var gs#9700 <= s_125_4
        fn_state.gs_9700 = s_125_4;
        // N s_125_6: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_126_0: const #1u : u8
        let s_126_0: bool = true;
        // D s_126_1: write-var gs#9702 <= s_126_0
        fn_state.gs_9702 = s_126_0;
        // N s_126_2: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_127_0: read-var fault.14:struct
        let s_127_0: bool = fn_state.fault._14;
        // D s_127_1: not s_127_0
        let s_127_1: bool = !s_127_0;
        // D s_127_2: write-var gs#9699 <= s_127_1
        fn_state.gs_9699 = s_127_1;
        // N s_127_3: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_128_0: read-var fault.16:struct
        let s_128_0: u32 = fn_state.fault._16;
        // C s_128_1: const #1u : u32
        let s_128_1: u32 = 1;
        // D s_128_2: cmp-eq s_128_0 s_128_1
        let s_128_2: bool = ((s_128_0) == (s_128_1));
        // N s_128_3: branch s_128_2 b137 b129
        if s_128_2 {
            return block_137(state, tracer, fn_state);
        } else {
            return block_129(state, tracer, fn_state);
        };
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_129_0: read-var fault.16:struct
        let s_129_0: u32 = fn_state.fault._16;
        // C s_129_1: const #6u : u32
        let s_129_1: u32 = 6;
        // D s_129_2: cmp-eq s_129_0 s_129_1
        let s_129_2: bool = ((s_129_0) == (s_129_1));
        // N s_129_3: branch s_129_2 b136 b130
        if s_129_2 {
            return block_136(state, tracer, fn_state);
        } else {
            return block_130(state, tracer, fn_state);
        };
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_130_0: read-var fault.16:struct
        let s_130_0: u32 = fn_state.fault._16;
        // C s_130_1: const #5u : u32
        let s_130_1: u32 = 5;
        // D s_130_2: cmp-eq s_130_0 s_130_1
        let s_130_2: bool = ((s_130_0) == (s_130_1));
        // D s_130_3: write-var gs#9706 <= s_130_2
        fn_state.gs_9706 = s_130_2;
        // N s_130_4: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_131_0: read-var gs#9706:u8
        let s_131_0: bool = fn_state.gs_9706;
        // D s_131_1: write-var gs#9707 <= s_131_0
        fn_state.gs_9707 = s_131_0;
        // N s_131_2: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_132_0: read-var gs#9707:u8
        let s_132_0: bool = fn_state.gs_9707;
        // N s_132_1: branch s_132_0 b135 b133
        if s_132_0 {
            return block_135(state, tracer, fn_state);
        } else {
            return block_133(state, tracer, fn_state);
        };
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // N s_133_0: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // N s_134_0: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_135_0: const #() : ()
        let s_135_0: () = ();
        // S s_135_1: call LS64InstructionSyndrome(s_135_0)
        let s_135_1: ProductType81b3b4c60b2f37ac = LS64InstructionSyndrome(
            state,
            tracer,
            s_135_0,
        );
        // D s_135_2: write-var ga#6805 <= s_135_1
        fn_state.ga_6805 = s_135_1;
        // D s_135_3: read-var ga#6805.0:struct
        let s_135_3: u32 = fn_state.ga_6805._0;
        // D s_135_4: read-var ga#6805.1:struct
        let s_135_4: u16 = fn_state.ga_6805._1;
        // D s_135_5: write-var iss2 <= s_135_3
        fn_state.iss2 = s_135_3;
        // C s_135_6: const #14s : i
        let s_135_6: i128 = 14;
        // D s_135_7: read-var iss:u25
        let s_135_7: u32 = fn_state.iss;
        // D s_135_8: cast zx s_135_7 -> bv
        let s_135_8: Bits = Bits::new(s_135_7 as u128, 25u16);
        // D s_135_9: cast zx s_135_4 -> bv
        let s_135_9: Bits = Bits::new(s_135_4 as u128, 11u16);
        // C s_135_10: const #10s : i
        let s_135_10: i128 = 10;
        // C s_135_11: const #1u : u64
        let s_135_11: u64 = 1;
        // C s_135_12: cast zx s_135_11 -> bv
        let s_135_12: Bits = Bits::new(s_135_11 as u128, 64u16);
        // C s_135_13: lsl s_135_12 s_135_10
        let s_135_13: Bits = s_135_12 << s_135_10;
        // C s_135_14: sub s_135_13 s_135_12
        let s_135_14: Bits = ((s_135_13) - (s_135_12));
        // D s_135_15: and s_135_9 s_135_14
        let s_135_15: Bits = ((s_135_9) & (s_135_14));
        // D s_135_16: lsl s_135_15 s_135_6
        let s_135_16: Bits = s_135_15 << s_135_6;
        // C s_135_17: lsl s_135_14 s_135_6
        let s_135_17: Bits = s_135_14 << s_135_6;
        // C s_135_18: cmpl s_135_17
        let s_135_18: Bits = !s_135_17;
        // D s_135_19: and s_135_8 s_135_18
        let s_135_19: Bits = ((s_135_8) & (s_135_18));
        // D s_135_20: or s_135_19 s_135_16
        let s_135_20: Bits = ((s_135_19) | (s_135_16));
        // D s_135_21: cast reint s_135_20 -> u25
        let s_135_21: u32 = (s_135_20.value() as u32);
        // D s_135_22: write-var iss <= s_135_21
        fn_state.iss = s_135_21;
        // N s_135_23: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_136_0: const #1u : u8
        let s_136_0: bool = true;
        // D s_136_1: write-var gs#9706 <= s_136_0
        fn_state.gs_9706 = s_136_0;
        // N s_136_2: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_137_0: const #1u : u8
        let s_137_0: bool = true;
        // D s_137_1: write-var gs#9707 <= s_137_0
        fn_state.gs_9707 = s_137_0;
        // N s_137_2: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_138_0: read-var fault.0:struct
        let s_138_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_138_1: write-var ga#6796 <= s_138_0
        fn_state.ga_6796 = s_138_0;
        // D s_138_2: read-var ga#6796.13:struct
        let s_138_2: bool = fn_state.ga_6796._13;
        // D s_138_3: write-var gs#9661 <= s_138_2
        fn_state.gs_9661 = s_138_2;
        // N s_138_4: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // C s_139_0: const #1u : u8
        let s_139_0: bool = true;
        // S s_139_1: call Bit(s_139_0)
        let s_139_1: bool = Bit(state, tracer, s_139_0);
        // C s_139_2: const #8s : i
        let s_139_2: i128 = 8;
        // D s_139_3: read-var iss2:u24
        let s_139_3: u32 = fn_state.iss2;
        // D s_139_4: cast zx s_139_3 -> bv
        let s_139_4: Bits = Bits::new(s_139_3 as u128, 24u16);
        // C s_139_5: const #1u : u64
        let s_139_5: u64 = 1;
        // D s_139_6: bit-insert s_139_4 s_139_4 s_139_2 s_139_5
        let s_139_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_139_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_139_4.length(),
            );
            (s_139_4 & mask) | (s_139_4 << s_139_2)
        };
        // D s_139_7: cast reint s_139_6 -> u24
        let s_139_7: u32 = (s_139_6.value() as u32);
        // D s_139_8: write-var iss2 <= s_139_7
        fn_state.iss2 = s_139_7;
        // N s_139_9: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_140_0: read-var fault:struct
        let s_140_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_140_1: call AArch64_PEErrorState(s_140_0)
        let s_140_1: u32 = AArch64_PEErrorState(state, tracer, s_140_0);
        // D s_140_2: call AArch64_EncodeSyncErrorSyndrome(s_140_1)
        let s_140_2: u8 = AArch64_EncodeSyncErrorSyndrome(state, tracer, s_140_1);
        // C s_140_3: const #11s : i
        let s_140_3: i128 = 11;
        // D s_140_4: read-var iss:u25
        let s_140_4: u32 = fn_state.iss;
        // D s_140_5: cast zx s_140_4 -> bv
        let s_140_5: Bits = Bits::new(s_140_4 as u128, 25u16);
        // D s_140_6: cast zx s_140_2 -> bv
        let s_140_6: Bits = Bits::new(s_140_2 as u128, 2u16);
        // C s_140_7: const #1s : i
        let s_140_7: i128 = 1;
        // C s_140_8: const #1u : u64
        let s_140_8: u64 = 1;
        // C s_140_9: cast zx s_140_8 -> bv
        let s_140_9: Bits = Bits::new(s_140_8 as u128, 64u16);
        // C s_140_10: lsl s_140_9 s_140_7
        let s_140_10: Bits = s_140_9 << s_140_7;
        // C s_140_11: sub s_140_10 s_140_9
        let s_140_11: Bits = ((s_140_10) - (s_140_9));
        // D s_140_12: and s_140_6 s_140_11
        let s_140_12: Bits = ((s_140_6) & (s_140_11));
        // D s_140_13: lsl s_140_12 s_140_3
        let s_140_13: Bits = s_140_12 << s_140_3;
        // C s_140_14: lsl s_140_11 s_140_3
        let s_140_14: Bits = s_140_11 << s_140_3;
        // C s_140_15: cmpl s_140_14
        let s_140_15: Bits = !s_140_14;
        // D s_140_16: and s_140_5 s_140_15
        let s_140_16: Bits = ((s_140_5) & (s_140_15));
        // D s_140_17: or s_140_16 s_140_13
        let s_140_17: Bits = ((s_140_16) | (s_140_13));
        // D s_140_18: cast reint s_140_17 -> u25
        let s_140_18: u32 = (s_140_17.value() as u32);
        // D s_140_19: write-var iss <= s_140_18
        fn_state.iss = s_140_18;
        // N s_140_20: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType44ac89053e6d35a9 {
        // D s_141_0: read-var fault.16:struct
        let s_141_0: u32 = fn_state.fault._16;
        // C s_141_1: const #8u : u32
        let s_141_1: u32 = 8;
        // D s_141_2: cmp-eq s_141_0 s_141_1
        let s_141_2: bool = ((s_141_0) == (s_141_1));
        // D s_141_3: write-var gs#9643 <= s_141_2
        fn_state.gs_9643 = s_141_2;
        // N s_141_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
