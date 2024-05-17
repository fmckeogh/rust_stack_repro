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
use HaveAccessFlagUpdateExt::*;
use HaveRME::*;
use AArch64_TakeException::*;
use HaveDoubleFaultExt::*;
use AArch64_RouteToSErrorOffset::*;
use u__UNKNOWN_bit::*;
use Zeros::*;
use HaveDirtyBitModifierExt::*;
use HaveAtomicExt::*;
use HaveNV2Ext::*;
use IsExternalAbort__1::*;
use ThisInstrAddr::*;
use IPAValid::*;
use EncodeLDFSC::*;
use Bit::*;
use EncodeGPCSC::*;
use common::*;
pub fn TakeGPCException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    vaddress: u64,
    fault: ProductType1d757adad216cdef,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_7195: ProductType9878976b5bcce9c9,
        gs_9937: bool,
        gs_9934: bool,
        ga_7224: ProductType9878976b5bcce9c9,
        ga_7270: bool,
        ga_7226: ProductType9878976b5bcce9c9,
        ga_7238: bool,
        except: ProductTypeb7f99f96751e17c4,
        ga_7246: ProductType9878976b5bcce9c9,
        ga_7208: ProductType9878976b5bcce9c9,
        preferred_exception_return: u64,
        ga_7191: bool,
        gs_9944: bool,
        vect_offset: i128,
        ga_7262: ProductType9878976b5bcce9c9,
        ga_7244: ProductType9878976b5bcce9c9,
        gs_9986: bool,
        gs_9933: bool,
        ga_7222: ProductType9878976b5bcce9c9,
        target_el: u8,
        ga_7180: ProductTypeda0231e9dc169f81,
        gs_9945: bool,
        ga_7177: ProductTypeda0231e9dc169f81,
        gs_9926: bool,
        ga_7242: ProductType9878976b5bcce9c9,
        ga_7181: ProductType9878976b5bcce9c9,
        gs_9938: bool,
        vaddress: u64,
        fault: ProductType1d757adad216cdef,
    }
    let fn_state = FunctionState {
        vaddress,
        fault,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaveRME(s_0_0)
        let s_0_1: bool = HaveRME(state, tracer, s_0_0);
        // N s_0_2: assert s_0_1
        let s_0_2: () = assert!(s_0_1);
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call HaveAtomicExt(s_0_3)
        let s_0_4: bool = HaveAtomicExt(state, tracer, s_0_3);
        // N s_0_5: assert s_0_4
        let s_0_5: () = assert!(s_0_4);
        // C s_0_6: const #() : ()
        let s_0_6: () = ();
        // S s_0_7: call HaveAccessFlagUpdateExt(s_0_6)
        let s_0_7: bool = HaveAccessFlagUpdateExt(state, tracer, s_0_6);
        // N s_0_8: assert s_0_7
        let s_0_8: () = assert!(s_0_7);
        // C s_0_9: const #() : ()
        let s_0_9: () = ();
        // S s_0_10: call HaveDirtyBitModifierExt(s_0_9)
        let s_0_10: bool = HaveDirtyBitModifierExt(state, tracer, s_0_9);
        // N s_0_11: assert s_0_10
        let s_0_11: () = assert!(s_0_10);
        // C s_0_12: const #() : ()
        let s_0_12: () = ();
        // S s_0_13: call HaveDoubleFaultExt(s_0_12)
        let s_0_13: bool = HaveDoubleFaultExt(state, tracer, s_0_12);
        // N s_0_14: assert s_0_13
        let s_0_14: () = assert!(s_0_13);
        // C s_0_15: const #35u : u32
        let s_0_15: u32 = 35;
        // D s_0_16: write-var except.1 <= s_0_15
        fn_state.except._1 = s_0_15;
        // C s_0_17: const #64s : i
        let s_0_17: i128 = 64;
        // D s_0_18: read-var vaddress:u64
        let s_0_18: u64 = fn_state.vaddress;
        // D s_0_19: cast zx s_0_18 -> bv
        let s_0_19: Bits = Bits::new(s_0_18 as u128, 64u16);
        // D s_0_20: bits-cast zx s_0_19 -> bv length s_0_17
        let s_0_20: Bits = s_0_19.zero_extend(s_0_17);
        // D s_0_21: cast reint s_0_20 -> u64
        let s_0_21: u64 = (s_0_20.value() as u64);
        // D s_0_22: write-var except.9 <= s_0_21
        fn_state.except._9 = s_0_21;
        // D s_0_23: read-var fault.12:struct
        let s_0_23: ProductTypeda0231e9dc169f81 = fn_state.fault._12;
        // D s_0_24: write-var except.4 <= s_0_23
        fn_state.except._4 = s_0_23;
        // C s_0_25: const #1u : u8
        let s_0_25: bool = true;
        // D s_0_26: write-var except.5 <= s_0_25
        fn_state.except._5 = s_0_25;
        // D s_0_27: read-var fault:struct
        let s_0_27: ProductType1d757adad216cdef = fn_state.fault;
        // D s_0_28: call IPAValid(s_0_27)
        let s_0_28: bool = IPAValid(state, tracer, s_0_27);
        // N s_0_29: branch s_0_28 b58 b1
        if s_0_28 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var except.3 <= s_1_0
        fn_state.except._3 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var fault.0:struct
        let s_2_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_2_1: write-var ga#7181 <= s_2_0
        fn_state.ga_7181 = s_2_0;
        // D s_2_2: read-var ga#7181.1:struct
        let s_2_2: u32 = fn_state.ga_7181._1;
        // C s_2_3: const #11u : u32
        let s_2_3: u32 = 11;
        // D s_2_4: cmp-eq s_2_2 s_2_3
        let s_2_4: bool = ((s_2_2) == (s_2_3));
        // N s_2_5: branch s_2_4 b57 b3
        if s_2_4 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #3s : i
        let s_4_0: i128 = 3;
        // S s_4_1: call Zeros(s_4_0)
        let s_4_1: Bits = Zeros(state, tracer, s_4_0);
        // D s_4_2: read-var except:struct
        let s_4_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_4_3: write-var except <= s_4_2
        fn_state.except = s_4_2;
        // D s_4_4: read-var fault.7:struct
        let s_4_4: bool = fn_state.fault._7;
        // N s_4_5: branch s_4_4 b56 b5
        if s_4_4 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var ga#7191 <= s_5_0
        fn_state.ga_7191 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#7191:u8
        let s_6_0: bool = fn_state.ga_7191;
        // D s_6_1: call Bit(s_6_0)
        let s_6_1: bool = Bit(state, tracer, s_6_0);
        // D s_6_2: read-var except:struct
        let s_6_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_6_3: write-var except <= s_6_2
        fn_state.except = s_6_2;
        // D s_6_4: read-var fault.0:struct
        let s_6_4: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_6_5: write-var ga#7195 <= s_6_4
        fn_state.ga_7195 = s_6_4;
        // D s_6_6: read-var ga#7195.1:struct
        let s_6_6: u32 = fn_state.ga_7195._1;
        // C s_6_7: const #0u : u32
        let s_6_7: u32 = 0;
        // D s_6_8: cmp-eq s_6_6 s_6_7
        let s_6_8: bool = ((s_6_6) == (s_6_7));
        // N s_6_9: branch s_6_8 b55 b7
        if s_6_8 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // S s_7_1: call Bit(s_7_0)
        let s_7_1: bool = Bit(state, tracer, s_7_0);
        // D s_7_2: read-var except:struct
        let s_7_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_7_3: write-var except <= s_7_2
        fn_state.except = s_7_2;
        // N s_7_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var fault.6:struct
        let s_8_0: ProductType396b95aa74979079 = fn_state.fault._6;
        // D s_8_1: call EncodeGPCSC(s_8_0)
        let s_8_1: u8 = EncodeGPCSC(state, tracer, s_8_0);
        // D s_8_2: read-var except:struct
        let s_8_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_8_3: write-var except <= s_8_2
        fn_state.except = s_8_2;
        // C s_8_4: const #() : ()
        let s_8_4: () = ();
        // S s_8_5: call HaveNV2Ext(s_8_4)
        let s_8_5: bool = HaveNV2Ext(state, tracer, s_8_4);
        // N s_8_6: branch s_8_5 b54 b9
        if s_8_5 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#9926 <= s_9_0
        fn_state.gs_9926 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#9926:u8
        let s_10_0: bool = fn_state.gs_9926;
        // N s_10_1: branch s_10_0 b53 b11
        if s_10_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // S s_11_1: call Bit(s_11_0)
        let s_11_1: bool = Bit(state, tracer, s_11_0);
        // D s_11_2: read-var except:struct
        let s_11_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_11_3: write-var except <= s_11_2
        fn_state.except = s_11_2;
        // N s_11_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var except:struct
        let s_12_0: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_12_1: write-var except <= s_12_0
        fn_state.except = s_12_0;
        // D s_12_2: read-var except:struct
        let s_12_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_12_3: write-var except <= s_12_2
        fn_state.except = s_12_2;
        // D s_12_4: read-var fault.0:struct
        let s_12_4: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_12_5: write-var ga#7222 <= s_12_4
        fn_state.ga_7222 = s_12_4;
        // D s_12_6: read-var ga#7222.1:struct
        let s_12_6: u32 = fn_state.ga_7222._1;
        // C s_12_7: const #6u : u32
        let s_12_7: u32 = 6;
        // D s_12_8: cmp-eq s_12_6 s_12_7
        let s_12_8: bool = ((s_12_6) == (s_12_7));
        // N s_12_9: branch s_12_8 b52 b13
        if s_12_8 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var fault.0:struct
        let s_13_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_13_1: write-var ga#7224 <= s_13_0
        fn_state.ga_7224 = s_13_0;
        // D s_13_2: read-var ga#7224.1:struct
        let s_13_2: u32 = fn_state.ga_7224._1;
        // C s_13_3: const #5u : u32
        let s_13_3: u32 = 5;
        // D s_13_4: cmp-eq s_13_2 s_13_3
        let s_13_4: bool = ((s_13_2) == (s_13_3));
        // N s_13_5: branch s_13_4 b51 b14
        if s_13_4 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var fault.0:struct
        let s_14_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_14_1: write-var ga#7226 <= s_14_0
        fn_state.ga_7226 = s_14_0;
        // D s_14_2: read-var ga#7226.1:struct
        let s_14_2: u32 = fn_state.ga_7226._1;
        // C s_14_3: const #8u : u32
        let s_14_3: u32 = 8;
        // D s_14_4: cmp-eq s_14_2 s_14_3
        let s_14_4: bool = ((s_14_2) == (s_14_3));
        // D s_14_5: write-var gs#9933 <= s_14_4
        fn_state.gs_9933 = s_14_4;
        // N s_14_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#9933:u8
        let s_15_0: bool = fn_state.gs_9933;
        // D s_15_1: write-var gs#9934 <= s_15_0
        fn_state.gs_9934 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#9934:u8
        let s_16_0: bool = fn_state.gs_9934;
        // N s_16_1: branch s_16_0 b50 b17
        if s_16_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // S s_17_1: call Bit(s_17_0)
        let s_17_1: bool = Bit(state, tracer, s_17_0);
        // D s_17_2: read-var except:struct
        let s_17_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_17_3: write-var except <= s_17_2
        fn_state.except = s_17_2;
        // N s_17_4: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var fault.14:struct
        let s_18_0: bool = fn_state.fault._14;
        // N s_18_1: branch s_18_0 b49 b19
        if s_18_0 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var ga#7238 <= s_19_0
        fn_state.ga_7238 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var ga#7238:u8
        let s_20_0: bool = fn_state.ga_7238;
        // D s_20_1: call Bit(s_20_0)
        let s_20_1: bool = Bit(state, tracer, s_20_0);
        // D s_20_2: read-var except:struct
        let s_20_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_20_3: write-var except <= s_20_2
        fn_state.except = s_20_2;
        // D s_20_4: read-var fault.0:struct
        let s_20_4: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_20_5: write-var ga#7242 <= s_20_4
        fn_state.ga_7242 = s_20_4;
        // D s_20_6: read-var ga#7242.1:struct
        let s_20_6: u32 = fn_state.ga_7242._1;
        // C s_20_7: const #6u : u32
        let s_20_7: u32 = 6;
        // D s_20_8: cmp-eq s_20_6 s_20_7
        let s_20_8: bool = ((s_20_6) == (s_20_7));
        // N s_20_9: branch s_20_8 b48 b21
        if s_20_8 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var fault.0:struct
        let s_21_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_21_1: write-var ga#7244 <= s_21_0
        fn_state.ga_7244 = s_21_0;
        // D s_21_2: read-var ga#7244.1:struct
        let s_21_2: u32 = fn_state.ga_7244._1;
        // C s_21_3: const #5u : u32
        let s_21_3: u32 = 5;
        // D s_21_4: cmp-eq s_21_2 s_21_3
        let s_21_4: bool = ((s_21_2) == (s_21_3));
        // N s_21_5: branch s_21_4 b47 b22
        if s_21_4 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var fault.0:struct
        let s_22_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_22_1: write-var ga#7246 <= s_22_0
        fn_state.ga_7246 = s_22_0;
        // D s_22_2: read-var ga#7246.1:struct
        let s_22_2: u32 = fn_state.ga_7246._1;
        // C s_22_3: const #8u : u32
        let s_22_3: u32 = 8;
        // D s_22_4: cmp-eq s_22_2 s_22_3
        let s_22_4: bool = ((s_22_2) == (s_22_3));
        // D s_22_5: write-var gs#9937 <= s_22_4
        fn_state.gs_9937 = s_22_4;
        // N s_22_6: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#9937:u8
        let s_23_0: bool = fn_state.gs_9937;
        // D s_23_1: write-var gs#9938 <= s_23_0
        fn_state.gs_9938 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#9938:u8
        let s_24_0: bool = fn_state.gs_9938;
        // N s_24_1: branch s_24_0 b46 b25
        if s_24_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var fault.16:struct
        let s_25_0: u32 = fn_state.fault._16;
        // C s_25_1: const #20u : u32
        let s_25_1: u32 = 20;
        // D s_25_2: cmp-eq s_25_0 s_25_1
        let s_25_2: bool = ((s_25_0) == (s_25_1));
        // N s_25_3: branch s_25_2 b45 b26
        if s_25_2 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var fault.16:struct
        let s_26_0: u32 = fn_state.fault._16;
        // C s_26_1: const #22u : u32
        let s_26_1: u32 = 22;
        // D s_26_2: cmp-eq s_26_0 s_26_1
        let s_26_2: bool = ((s_26_0) == (s_26_1));
        // D s_26_3: write-var gs#9944 <= s_26_2
        fn_state.gs_9944 = s_26_2;
        // N s_26_4: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#9944:u8
        let s_27_0: bool = fn_state.gs_9944;
        // N s_27_1: branch s_27_0 b44 b28
        if s_27_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var fault.0:struct
        let s_28_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_28_1: write-var ga#7262 <= s_28_0
        fn_state.ga_7262 = s_28_0;
        // D s_28_2: read-var ga#7262.4:struct
        let s_28_2: bool = fn_state.ga_7262._4;
        // N s_28_3: branch s_28_2 b43 b29
        if s_28_2 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#9945 <= s_29_0
        fn_state.gs_9945 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#9945:u8
        let s_30_0: bool = fn_state.gs_9945;
        // N s_30_1: branch s_30_0 b42 b31
        if s_30_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var fault.19:struct
        let s_31_0: bool = fn_state.fault._19;
        // N s_31_1: branch s_31_0 b41 b32
        if s_31_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var ga#7270 <= s_32_0
        fn_state.ga_7270 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var ga#7270:u8
        let s_33_0: bool = fn_state.ga_7270;
        // D s_33_1: call Bit(s_33_0)
        let s_33_1: bool = Bit(state, tracer, s_33_0);
        // D s_33_2: read-var except:struct
        let s_33_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_33_3: write-var except <= s_33_2
        fn_state.except = s_33_2;
        // N s_33_4: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var fault.16:struct
        let s_34_0: u32 = fn_state.fault._16;
        // D s_34_1: read-var fault.9:struct
        let s_34_1: i128 = fn_state.fault._9;
        // D s_34_2: call EncodeLDFSC(s_34_0, s_34_1)
        let s_34_2: u8 = EncodeLDFSC(state, tracer, s_34_0, s_34_1);
        // D s_34_3: read-var except:struct
        let s_34_3: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_34_4: write-var except <= s_34_3
        fn_state.except = s_34_3;
        // C s_34_5: const #64s : i64
        let s_34_5: i64 = 64;
        // C s_34_6: cast zx s_34_5 -> i
        let s_34_6: i128 = (i128::try_from(s_34_5).unwrap());
        // S s_34_7: call ThisInstrAddr(s_34_6)
        let s_34_7: Bits = ThisInstrAddr(state, tracer, s_34_6);
        // S s_34_8: cast reint s_34_7 -> u64
        let s_34_8: u64 = (s_34_7.value() as u64);
        // D s_34_9: write-var preferred_exception_return <= s_34_8
        fn_state.preferred_exception_return = s_34_8;
        // C s_34_10: const #424u : u32
        let s_34_10: u32 = 424;
        // D s_34_11: read-reg s_34_10:u8
        let s_34_11: u8 = {
            let value = state.read_register::<u8>(s_34_10 as isize);
            tracer.read_register(s_34_10 as isize, value);
            value
        };
        // D s_34_12: write-var target_el <= s_34_11
        fn_state.target_el = s_34_11;
        // D s_34_13: read-var fault:struct
        let s_34_13: ProductType1d757adad216cdef = fn_state.fault;
        // D s_34_14: call IsExternalAbort__1(s_34_13)
        let s_34_14: bool = IsExternalAbort__1(state, tracer, s_34_13);
        // N s_34_15: branch s_34_14 b40 b35
        if s_34_14 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var gs#9986 <= s_35_0
        fn_state.gs_9986 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#9986:u8
        let s_36_0: bool = fn_state.gs_9986;
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
    ) -> () {
        // C s_37_0: const #0u : u8
        let s_37_0: u8 = 0;
        // C s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 4u16);
        // C s_37_2: cast zx s_37_1 -> i
        let s_37_2: i128 = (s_37_1.value() as i128);
        // D s_37_3: write-var vect_offset <= s_37_2
        fn_state.vect_offset = s_37_2;
        // N s_37_4: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var vect_offset:i
        let s_38_0: i128 = fn_state.vect_offset;
        // D s_38_1: read-var target_el:u8
        let s_38_1: u8 = fn_state.target_el;
        // D s_38_2: read-var except:struct
        let s_38_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_38_3: read-var preferred_exception_return:u64
        let s_38_3: u64 = fn_state.preferred_exception_return;
        // D s_38_4: call AArch64_TakeException(s_38_1, s_38_2, s_38_3, s_38_0)
        let s_38_4: () = AArch64_TakeException(
            state,
            tracer,
            s_38_1,
            s_38_2,
            s_38_3,
            s_38_0,
        );
        // N s_38_5: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #384u : u12
        let s_39_0: u16 = 384;
        // C s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 12u16);
        // C s_39_2: cast zx s_39_1 -> i
        let s_39_2: i128 = (s_39_1.value() as i128);
        // D s_39_3: write-var vect_offset <= s_39_2
        fn_state.vect_offset = s_39_2;
        // N s_39_4: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var target_el:u8
        let s_40_0: u8 = fn_state.target_el;
        // D s_40_1: call AArch64_RouteToSErrorOffset(s_40_0)
        let s_40_1: bool = AArch64_RouteToSErrorOffset(state, tracer, s_40_0);
        // D s_40_2: write-var gs#9986 <= s_40_1
        fn_state.gs_9986 = s_40_1;
        // N s_40_3: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var ga#7270 <= s_41_0
        fn_state.ga_7270 = s_41_0;
        // N s_41_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call __UNKNOWN_bit(s_42_0)
        let s_42_1: bool = u__UNKNOWN_bit(state, tracer, s_42_0);
        // S s_42_2: call Bit(s_42_1)
        let s_42_2: bool = Bit(state, tracer, s_42_1);
        // D s_42_3: read-var except:struct
        let s_42_3: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_42_4: write-var except <= s_42_3
        fn_state.except = s_42_3;
        // N s_42_5: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var fault:struct
        let s_43_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_43_1: call IsExternalAbort__1(s_43_0)
        let s_43_1: bool = IsExternalAbort__1(state, tracer, s_43_0);
        // D s_43_2: write-var gs#9945 <= s_43_1
        fn_state.gs_9945 = s_43_1;
        // N s_43_3: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #() : ()
        let s_44_0: () = ();
        // S s_44_1: call __UNKNOWN_bit(s_44_0)
        let s_44_1: bool = u__UNKNOWN_bit(state, tracer, s_44_0);
        // S s_44_2: call Bit(s_44_1)
        let s_44_2: bool = Bit(state, tracer, s_44_1);
        // D s_44_3: read-var except:struct
        let s_44_3: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_44_4: write-var except <= s_44_3
        fn_state.except = s_44_3;
        // N s_44_5: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #1u : u8
        let s_45_0: bool = true;
        // D s_45_1: write-var gs#9944 <= s_45_0
        fn_state.gs_9944 = s_45_0;
        // N s_45_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #1u : u8
        let s_46_0: bool = true;
        // S s_46_1: call Bit(s_46_0)
        let s_46_1: bool = Bit(state, tracer, s_46_0);
        // D s_46_2: read-var except:struct
        let s_46_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_46_3: write-var except <= s_46_2
        fn_state.except = s_46_2;
        // N s_46_4: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #1u : u8
        let s_47_0: bool = true;
        // D s_47_1: write-var gs#9937 <= s_47_0
        fn_state.gs_9937 = s_47_0;
        // N s_47_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #1u : u8
        let s_48_0: bool = true;
        // D s_48_1: write-var gs#9938 <= s_48_0
        fn_state.gs_9938 = s_48_0;
        // N s_48_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #1u : u8
        let s_49_0: bool = true;
        // D s_49_1: write-var ga#7238 <= s_49_0
        fn_state.ga_7238 = s_49_0;
        // N s_49_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #1u : u8
        let s_50_0: bool = true;
        // S s_50_1: call Bit(s_50_0)
        let s_50_1: bool = Bit(state, tracer, s_50_0);
        // D s_50_2: read-var except:struct
        let s_50_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_50_3: write-var except <= s_50_2
        fn_state.except = s_50_2;
        // N s_50_4: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #1u : u8
        let s_51_0: bool = true;
        // D s_51_1: write-var gs#9933 <= s_51_0
        fn_state.gs_9933 = s_51_0;
        // N s_51_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #1u : u8
        let s_52_0: bool = true;
        // D s_52_1: write-var gs#9934 <= s_52_0
        fn_state.gs_9934 = s_52_0;
        // N s_52_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #1u : u8
        let s_53_0: bool = true;
        // S s_53_1: call Bit(s_53_0)
        let s_53_1: bool = Bit(state, tracer, s_53_0);
        // D s_53_2: read-var except:struct
        let s_53_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_53_3: write-var except <= s_53_2
        fn_state.except = s_53_2;
        // N s_53_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var fault.0:struct
        let s_54_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_54_1: write-var ga#7208 <= s_54_0
        fn_state.ga_7208 = s_54_0;
        // D s_54_2: read-var ga#7208.1:struct
        let s_54_2: u32 = fn_state.ga_7208._1;
        // C s_54_3: const #9u : u32
        let s_54_3: u32 = 9;
        // D s_54_4: cmp-eq s_54_2 s_54_3
        let s_54_4: bool = ((s_54_2) == (s_54_3));
        // D s_54_5: write-var gs#9926 <= s_54_4
        fn_state.gs_9926 = s_54_4;
        // N s_54_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #1u : u8
        let s_55_0: bool = true;
        // S s_55_1: call Bit(s_55_0)
        let s_55_1: bool = Bit(state, tracer, s_55_0);
        // D s_55_2: read-var except:struct
        let s_55_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_55_3: write-var except <= s_55_2
        fn_state.except = s_55_2;
        // N s_55_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #1u : u8
        let s_56_0: bool = true;
        // D s_56_1: write-var ga#7191 <= s_56_0
        fn_state.ga_7191 = s_56_0;
        // N s_56_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #1u : u8
        let s_57_0: bool = true;
        // S s_57_1: call Bit(s_57_0)
        let s_57_1: bool = Bit(state, tracer, s_57_0);
        // D s_57_2: read-var except:struct
        let s_57_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_57_3: write-var except <= s_57_2
        fn_state.except = s_57_2;
        // N s_57_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #1u : u8
        let s_58_0: bool = true;
        // D s_58_1: write-var except.3 <= s_58_0
        fn_state.except._3 = s_58_0;
        // D s_58_2: read-var fault.8:struct
        let s_58_2: ProductTypeda0231e9dc169f81 = fn_state.fault._8;
        // D s_58_3: write-var ga#7177 <= s_58_2
        fn_state.ga_7177 = s_58_2;
        // D s_58_4: read-var ga#7177.1:struct
        let s_58_4: u32 = fn_state.ga_7177._1;
        // C s_58_5: const #0u : u32
        let s_58_5: u32 = 0;
        // D s_58_6: cmp-eq s_58_4 s_58_5
        let s_58_6: bool = ((s_58_4) == (s_58_5));
        // N s_58_7: branch s_58_6 b61 b59
        if s_58_6 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #0u : u8
        let s_59_0: bool = false;
        // D s_59_1: write-var except.0 <= s_59_0
        fn_state.except._0 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var fault.8:struct
        let s_60_0: ProductTypeda0231e9dc169f81 = fn_state.fault._8;
        // D s_60_1: write-var ga#7180 <= s_60_0
        fn_state.ga_7180 = s_60_0;
        // D s_60_2: read-var ga#7180.0:struct
        let s_60_2: u64 = fn_state.ga_7180._0;
        // D s_60_3: write-var except.2 <= s_60_2
        fn_state.except._2 = s_60_2;
        // N s_60_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #1u : u8
        let s_61_0: bool = true;
        // D s_61_1: write-var except.0 <= s_61_0
        fn_state.except._0 = s_61_0;
        // N s_61_2: jump b60
        return block_60(state, tracer, fn_state);
    }
}
