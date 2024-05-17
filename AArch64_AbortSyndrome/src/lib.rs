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
use HavePFAR::*;
use u__UNKNOWN_bits::*;
use HaveMTE4Ext::*;
use IPAValid::*;
use IsExternalSyncAbort__1::*;
use ExceptionSyndrome::*;
use AArch64_FaultSyndrome::*;
use u__IMPDEF_boolean::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_VM::*;
use common::*;
pub fn AArch64_AbortSyndrome<T: Tracer>(
    state: &mut State,
    tracer: &T,
    exceptype: u32,
    fault: ProductType1d757adad216cdef,
    vaddress: u64,
    target_el: u8,
) -> ProductTypeb7f99f96751e17c4 {
    #[derive(Default)]
    struct FunctionState {
        except: ProductTypeb7f99f96751e17c4,
        gs_9724: bool,
        ga_6934: ProductTypeda0231e9dc169f81,
        gs_9726: bool,
        ga_6931: ProductTypeda0231e9dc169f81,
        gs_9723: bool,
        gs_9722: bool,
        gs_9721: bool,
        d_side: bool,
        ga_6923: ProductType44ac89053e6d35a9,
        gs_9725: bool,
        gs_9720: bool,
        exceptype: u32,
        fault: ProductType1d757adad216cdef,
        vaddress: u64,
        target_el: u8,
    }
    let fn_state = FunctionState {
        exceptype,
        fault,
        vaddress,
        target_el,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_0_0: read-var exceptype:u32
        let s_0_0: u32 = fn_state.exceptype;
        // D s_0_1: call ExceptionSyndrome(s_0_0)
        let s_0_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(state, tracer, s_0_0);
        // D s_0_2: write-var except <= s_0_1
        fn_state.except = s_0_1;
        // D s_0_3: read-var exceptype:u32
        let s_0_3: u32 = fn_state.exceptype;
        // C s_0_4: const #19u : u32
        let s_0_4: u32 = 19;
        // D s_0_5: cmp-eq s_0_3 s_0_4
        let s_0_5: bool = ((s_0_3) == (s_0_4));
        // N s_0_6: branch s_0_5 b35 b1
        if s_0_5 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_1_0: read-var exceptype:u32
        let s_1_0: u32 = fn_state.exceptype;
        // C s_1_1: const #20u : u32
        let s_1_1: u32 = 20;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // N s_1_3: branch s_1_2 b34 b2
        if s_1_2 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_2_0: read-var exceptype:u32
        let s_2_0: u32 = fn_state.exceptype;
        // C s_2_1: const #27u : u32
        let s_2_1: u32 = 27;
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // N s_2_3: branch s_2_2 b33 b3
        if s_2_2 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_3_0: read-var exceptype:u32
        let s_3_0: u32 = fn_state.exceptype;
        // C s_3_1: const #28u : u32
        let s_3_1: u32 = 28;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // D s_3_3: write-var gs#9720 <= s_3_2
        fn_state.gs_9720 = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_4_0: read-var gs#9720:u8
        let s_4_0: bool = fn_state.gs_9720;
        // D s_4_1: write-var gs#9721 <= s_4_0
        fn_state.gs_9721 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_5_0: read-var gs#9721:u8
        let s_5_0: bool = fn_state.gs_9721;
        // D s_5_1: write-var gs#9722 <= s_5_0
        fn_state.gs_9722 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_6_0: read-var gs#9722:u8
        let s_6_0: bool = fn_state.gs_9722;
        // D s_6_1: write-var d_side <= s_6_0
        fn_state.d_side = s_6_0;
        // C s_6_2: const #() : ()
        let s_6_2: () = ();
        // S s_6_3: call HavePFAR(s_6_2)
        let s_6_3: bool = HavePFAR(state, tracer, s_6_2);
        // S s_6_4: not s_6_3
        let s_6_4: bool = !s_6_3;
        // N s_6_5: branch s_6_4 b32 b7
        if s_6_4 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_7_0: read-var fault:struct
        let s_7_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_7_1: call IsExternalSyncAbort__1(s_7_0)
        let s_7_1: bool = IsExternalSyncAbort__1(state, tracer, s_7_0);
        // D s_7_2: not s_7_1
        let s_7_2: bool = !s_7_1;
        // D s_7_3: write-var gs#9723 <= s_7_2
        fn_state.gs_9723 = s_7_2;
        // N s_7_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_8_0: read-var gs#9723:u8
        let s_8_0: bool = fn_state.gs_9723;
        // N s_8_1: branch s_8_0 b31 b9
        if s_8_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call EL2Enabled(s_9_0)
        let s_9_1: bool = EL2Enabled(state, tracer, s_9_0);
        // N s_9_2: branch s_9_1 b30 b10
        if s_9_1 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#9724 <= s_10_0
        fn_state.gs_9724 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_11_0: read-var gs#9724:u8
        let s_11_0: bool = fn_state.gs_9724;
        // N s_11_1: branch s_11_0 b29 b12
        if s_11_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#9725 <= s_12_0
        fn_state.gs_9725 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_13_0: read-var gs#9725:u8
        let s_13_0: bool = fn_state.gs_9725;
        // D s_13_1: write-var gs#9726 <= s_13_0
        fn_state.gs_9726 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_14_0: read-var gs#9726:u8
        let s_14_0: bool = fn_state.gs_9726;
        // N s_14_1: branch s_14_0 b28 b15
        if s_14_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_15_0: const #"PFAR_ELx is valid" : str
        let s_15_0: &'static str = "PFAR_ELx is valid";
        // S s_15_1: call __IMPDEF_boolean(s_15_0)
        let s_15_1: bool = u__IMPDEF_boolean(state, tracer, s_15_0);
        // D s_15_2: write-var except.5 <= s_15_1
        fn_state.except._5 = s_15_1;
        // N s_15_3: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_16_0: read-var except.5:struct
        let s_16_0: bool = fn_state.except._5;
        // D s_16_1: read-var d_side:u8
        let s_16_1: bool = fn_state.d_side;
        // D s_16_2: read-var fault:struct
        let s_16_2: ProductType1d757adad216cdef = fn_state.fault;
        // D s_16_3: call AArch64_FaultSyndrome(s_16_1, s_16_2, s_16_0)
        let s_16_3: ProductType44ac89053e6d35a9 = AArch64_FaultSyndrome(
            state,
            tracer,
            s_16_1,
            s_16_2,
            s_16_0,
        );
        // D s_16_4: write-var ga#6923 <= s_16_3
        fn_state.ga_6923 = s_16_3;
        // D s_16_5: read-var ga#6923.0:struct
        let s_16_5: u32 = fn_state.ga_6923._0;
        // D s_16_6: read-var ga#6923.1:struct
        let s_16_6: u32 = fn_state.ga_6923._1;
        // D s_16_7: write-var except.6 <= s_16_5
        fn_state.except._6 = s_16_5;
        // D s_16_8: write-var except.7 <= s_16_6
        fn_state.except._7 = s_16_6;
        // D s_16_9: read-var fault.16:struct
        let s_16_9: u32 = fn_state.fault._16;
        // C s_16_10: const #16u : u32
        let s_16_10: u32 = 16;
        // D s_16_11: cmp-eq s_16_9 s_16_10
        let s_16_11: bool = ((s_16_9) == (s_16_10));
        // N s_16_12: branch s_16_11 b25 b17
        if s_16_11 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_17_0: const #64s : i
        let s_17_0: i128 = 64;
        // D s_17_1: read-var vaddress:u64
        let s_17_1: u64 = fn_state.vaddress;
        // D s_17_2: cast zx s_17_1 -> bv
        let s_17_2: Bits = Bits::new(s_17_1 as u128, 64u16);
        // D s_17_3: bits-cast zx s_17_2 -> bv length s_17_0
        let s_17_3: Bits = s_17_2.zero_extend(s_17_0);
        // D s_17_4: cast reint s_17_3 -> u64
        let s_17_4: u64 = (s_17_3.value() as u64);
        // D s_17_5: write-var except.9 <= s_17_4
        fn_state.except._9 = s_17_4;
        // N s_17_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_18_0: read-var fault:struct
        let s_18_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_18_1: call IPAValid(s_18_0)
        let s_18_1: bool = IPAValid(state, tracer, s_18_0);
        // N s_18_2: branch s_18_1 b21 b19
        if s_18_1 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var except.3 <= s_19_0
        fn_state.except._3 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_20_0: read-var except:struct
        let s_20_0: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // N s_20_1: return s_20_0
        return s_20_0;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var except.3 <= s_21_0
        fn_state.except._3 = s_21_0;
        // D s_21_2: read-var fault.8:struct
        let s_21_2: ProductTypeda0231e9dc169f81 = fn_state.fault._8;
        // D s_21_3: write-var ga#6931 <= s_21_2
        fn_state.ga_6931 = s_21_2;
        // D s_21_4: read-var ga#6931.1:struct
        let s_21_4: u32 = fn_state.ga_6931._1;
        // C s_21_5: const #0u : u32
        let s_21_5: u32 = 0;
        // D s_21_6: cmp-eq s_21_4 s_21_5
        let s_21_6: bool = ((s_21_4) == (s_21_5));
        // N s_21_7: branch s_21_6 b24 b22
        if s_21_6 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var except.0 <= s_22_0
        fn_state.except._0 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_23_0: read-var fault.8:struct
        let s_23_0: ProductTypeda0231e9dc169f81 = fn_state.fault._8;
        // D s_23_1: write-var ga#6934 <= s_23_0
        fn_state.ga_6934 = s_23_0;
        // D s_23_2: read-var ga#6934.0:struct
        let s_23_2: u64 = fn_state.ga_6934._0;
        // D s_23_3: write-var except.2 <= s_23_2
        fn_state.except._2 = s_23_2;
        // N s_23_4: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var except.0 <= s_24_0
        fn_state.except._0 = s_24_0;
        // N s_24_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call HaveMTE4Ext(s_25_0)
        let s_25_1: bool = HaveMTE4Ext(state, tracer, s_25_0);
        // N s_25_2: branch s_25_1 b27 b26
        if s_25_1 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_26_0: const #4s : i64
        let s_26_0: i64 = 4;
        // C s_26_1: cast zx s_26_0 -> i
        let s_26_1: i128 = (i128::try_from(s_26_0).unwrap());
        // S s_26_2: call __UNKNOWN_bits(s_26_1)
        let s_26_2: Bits = u__UNKNOWN_bits(state, tracer, s_26_1);
        // S s_26_3: cast reint s_26_2 -> u8
        let s_26_3: u8 = (s_26_2.value() as u8);
        // C s_26_4: const #0s : i
        let s_26_4: i128 = 0;
        // D s_26_5: read-var vaddress:u64
        let s_26_5: u64 = fn_state.vaddress;
        // D s_26_6: cast zx s_26_5 -> bv
        let s_26_6: Bits = Bits::new(s_26_5 as u128, 64u16);
        // C s_26_7: const #1s : i64
        let s_26_7: i64 = 1;
        // C s_26_8: cast zx s_26_7 -> i
        let s_26_8: i128 = (i128::try_from(s_26_7).unwrap());
        // C s_26_9: const #59s : i
        let s_26_9: i128 = 59;
        // C s_26_10: add s_26_9 s_26_8
        let s_26_10: i128 = (s_26_9 + s_26_8);
        // D s_26_11: bit-extract s_26_6 s_26_4 s_26_10
        let s_26_11: Bits = (Bits::new(
            ((s_26_6) >> (s_26_4)).value(),
            u16::try_from(s_26_10).unwrap(),
        ));
        // D s_26_12: cast reint s_26_11 -> u60
        let s_26_12: u64 = (s_26_11.value() as u64);
        // S s_26_13: cast zx s_26_3 -> bv
        let s_26_13: Bits = Bits::new(s_26_3 as u128, 4u16);
        // D s_26_14: cast zx s_26_12 -> bv
        let s_26_14: Bits = Bits::new(s_26_12 as u128, 60u16);
        // S s_26_15: cast reint s_26_13 -> u128
        let s_26_15: u128 = (s_26_13.value() as u128);
        // D s_26_16: size-of s_26_13
        let s_26_16: u16 = s_26_13.length();
        // D s_26_17: cast reint s_26_14 -> u128
        let s_26_17: u128 = (s_26_14.value() as u128);
        // D s_26_18: size-of s_26_14
        let s_26_18: u16 = s_26_14.length();
        // D s_26_19: lsl s_26_15 s_26_18
        let s_26_19: u128 = s_26_15 << s_26_18;
        // D s_26_20: or s_26_19 s_26_17
        let s_26_20: u128 = ((s_26_19) | (s_26_17));
        // D s_26_21: add s_26_16 s_26_18
        let s_26_21: u16 = (s_26_16 + s_26_18);
        // D s_26_22: create-bits s_26_20 s_26_21
        let s_26_22: Bits = Bits::new(s_26_20, s_26_21);
        // D s_26_23: cast reint s_26_22 -> u64
        let s_26_23: u64 = (s_26_22.value() as u64);
        // D s_26_24: write-var except.9 <= s_26_23
        fn_state.except._9 = s_26_23;
        // N s_26_25: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_27_0: const #64s : i
        let s_27_0: i128 = 64;
        // D s_27_1: read-var vaddress:u64
        let s_27_1: u64 = fn_state.vaddress;
        // D s_27_2: cast zx s_27_1 -> bv
        let s_27_2: Bits = Bits::new(s_27_1 as u128, 64u16);
        // D s_27_3: bits-cast zx s_27_2 -> bv length s_27_0
        let s_27_3: Bits = s_27_2.zero_extend(s_27_0);
        // D s_27_4: cast reint s_27_3 -> u64
        let s_27_4: u64 = (s_27_3.value() as u64);
        // D s_27_5: write-var except.9 <= s_27_4
        fn_state.except._9 = s_27_4;
        // N s_27_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var except.5 <= s_28_0
        fn_state.except._5 = s_28_0;
        // N s_28_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_29_0: read-var target_el:u8
        let s_29_0: u8 = fn_state.target_el;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 2u16);
        // C s_29_2: const #440u : u32
        let s_29_2: u32 = 440;
        // D s_29_3: read-reg s_29_2:u8
        let s_29_3: u8 = {
            let value = state.read_register::<u8>(s_29_2 as isize);
            tracer.read_register(s_29_2 as isize, value);
            value
        };
        // D s_29_4: cast zx s_29_3 -> bv
        let s_29_4: Bits = Bits::new(s_29_3 as u128, 2u16);
        // D s_29_5: cmp-eq s_29_1 s_29_4
        let s_29_5: bool = ((s_29_1) == (s_29_4));
        // D s_29_6: write-var gs#9725 <= s_29_5
        fn_state.gs_9725 = s_29_5;
        // N s_29_7: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_30_0: const #102552u : u32
        let s_30_0: u32 = 102552;
        // D s_30_1: read-reg s_30_0:struct
        let s_30_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // D s_30_2: call _get_HCR_EL2_Type_VM(s_30_1)
        let s_30_2: bool = u_get_HCR_EL2_Type_VM(state, tracer, s_30_1);
        // D s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // C s_30_4: const #1u : u8
        let s_30_4: bool = true;
        // C s_30_5: cast zx s_30_4 -> bv
        let s_30_5: Bits = Bits::new(s_30_4 as u128, 1u16);
        // D s_30_6: cmp-eq s_30_3 s_30_5
        let s_30_6: bool = ((s_30_3) == (s_30_5));
        // D s_30_7: write-var gs#9724 <= s_30_6
        fn_state.gs_9724 = s_30_6;
        // N s_30_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // D s_31_1: write-var gs#9726 <= s_31_0
        fn_state.gs_9726 = s_31_0;
        // N s_31_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_32_0: const #1u : u8
        let s_32_0: bool = true;
        // D s_32_1: write-var gs#9723 <= s_32_0
        fn_state.gs_9723 = s_32_0;
        // N s_32_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_33_0: const #1u : u8
        let s_33_0: bool = true;
        // D s_33_1: write-var gs#9720 <= s_33_0
        fn_state.gs_9720 = s_33_0;
        // N s_33_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // D s_34_1: write-var gs#9721 <= s_34_0
        fn_state.gs_9721 = s_34_0;
        // N s_34_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var gs#9722 <= s_35_0
        fn_state.gs_9722 = s_35_0;
        // N s_35_2: jump b6
        return block_6(state, tracer, fn_state);
    }
}
