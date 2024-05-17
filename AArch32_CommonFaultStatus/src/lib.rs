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
use EncodeSDFSC::*;
use AArch32_EncodeAsyncErrorSyndrome::*;
use AArch32_PEErrorState::*;
use EncodeLDFSC::*;
use Bit::*;
use Zeros::*;
use HaveRASExt::*;
use IsExternalAbort__1::*;
use common::*;
pub fn AArch32_CommonFaultStatus<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault: ProductType1d757adad216cdef,
    long_format: bool,
) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        target: u32,
        gs_9223: bool,
        ga_6428: bool,
        fault: ProductType1d757adad216cdef,
        long_format: bool,
    }
    let fn_state = FunctionState {
        fault,
        long_format,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_0_0: const #32s : i
        let s_0_0: i128 = 32;
        // S s_0_1: call Zeros(s_0_0)
        let s_0_1: Bits = Zeros(state, tracer, s_0_0);
        // S s_0_2: cast reint s_0_1 -> u32
        let s_0_2: u32 = (s_0_1.value() as u32);
        // D s_0_3: write-var target <= s_0_2
        fn_state.target = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call HaveRASExt(s_0_4)
        let s_0_5: bool = HaveRASExt(state, tracer, s_0_4);
        // N s_0_6: branch s_0_5 b15 b1
        if s_0_5 {
            return block_15(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#9223 <= s_1_0
        fn_state.gs_9223 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_2_0: read-var gs#9223:u8
        let s_2_0: bool = fn_state.gs_9223;
        // N s_2_1: branch s_2_0 b14 b3
        if s_2_0 {
            return block_14(state, tracer, fn_state);
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
        // D s_4_0: read-var fault:struct
        let s_4_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_4_1: call IsExternalAbort__1(s_4_0)
        let s_4_1: bool = IsExternalAbort__1(state, tracer, s_4_0);
        // N s_4_2: branch s_4_1 b13 b5
        if s_4_1 {
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
        // D s_6_0: read-var long_format:u8
        let s_6_0: bool = fn_state.long_format;
        // N s_6_1: branch s_6_0 b12 b7
        if s_6_0 {
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
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var ga#6428 <= s_7_0
        fn_state.ga_6428 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_8_0: read-var ga#6428:u8
        let s_8_0: bool = fn_state.ga_6428;
        // D s_8_1: call Bit(s_8_0)
        let s_8_1: bool = Bit(state, tracer, s_8_0);
        // C s_8_2: const #9s : i
        let s_8_2: i128 = 9;
        // D s_8_3: read-var target:u32
        let s_8_3: u32 = fn_state.target;
        // D s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 32u16);
        // C s_8_5: const #1u : u64
        let s_8_5: u64 = 1;
        // D s_8_6: bit-insert s_8_4 s_8_4 s_8_2 s_8_5
        let s_8_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_8_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_8_4.length(),
            );
            (s_8_4 & mask) | (s_8_4 << s_8_2)
        };
        // D s_8_7: cast reint s_8_6 -> u32
        let s_8_7: u32 = (s_8_6.value() as u32);
        // D s_8_8: write-var target <= s_8_7
        fn_state.target = s_8_7;
        // D s_8_9: read-var long_format:u8
        let s_8_9: bool = fn_state.long_format;
        // N s_8_10: branch s_8_9 b11 b9
        if s_8_9 {
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
        // D s_9_0: read-var fault.16:struct
        let s_9_0: u32 = fn_state.fault._16;
        // D s_9_1: read-var fault.9:struct
        let s_9_1: i128 = fn_state.fault._9;
        // D s_9_2: call EncodeSDFSC(s_9_0, s_9_1)
        let s_9_2: u8 = EncodeSDFSC(state, tracer, s_9_0, s_9_1);
        // C s_9_3: const #4s : i
        let s_9_3: i128 = 4;
        // D s_9_4: cast zx s_9_2 -> bv
        let s_9_4: Bits = Bits::new(s_9_2 as u128, 5u16);
        // C s_9_5: const #1s : i64
        let s_9_5: i64 = 1;
        // C s_9_6: cast zx s_9_5 -> i
        let s_9_6: i128 = (i128::try_from(s_9_5).unwrap());
        // C s_9_7: const #0s : i
        let s_9_7: i128 = 0;
        // C s_9_8: add s_9_7 s_9_6
        let s_9_8: i128 = (s_9_7 + s_9_6);
        // D s_9_9: bit-extract s_9_4 s_9_3 s_9_8
        let s_9_9: Bits = (Bits::new(
            ((s_9_4) >> (s_9_3)).value(),
            u16::try_from(s_9_8).unwrap(),
        ));
        // D s_9_10: cast reint s_9_9 -> u8
        let s_9_10: bool = ((s_9_9.value()) != 0);
        // C s_9_11: const #10s : i
        let s_9_11: i128 = 10;
        // D s_9_12: read-var target:u32
        let s_9_12: u32 = fn_state.target;
        // D s_9_13: cast zx s_9_12 -> bv
        let s_9_13: Bits = Bits::new(s_9_12 as u128, 32u16);
        // D s_9_14: cast zx s_9_10 -> bv
        let s_9_14: Bits = Bits::new(s_9_10 as u128, 1u16);
        // C s_9_15: const #0s : i
        let s_9_15: i128 = 0;
        // C s_9_16: const #1u : u64
        let s_9_16: u64 = 1;
        // C s_9_17: cast zx s_9_16 -> bv
        let s_9_17: Bits = Bits::new(s_9_16 as u128, 64u16);
        // C s_9_18: lsl s_9_17 s_9_15
        let s_9_18: Bits = s_9_17 << s_9_15;
        // C s_9_19: sub s_9_18 s_9_17
        let s_9_19: Bits = ((s_9_18) - (s_9_17));
        // D s_9_20: and s_9_14 s_9_19
        let s_9_20: Bits = ((s_9_14) & (s_9_19));
        // D s_9_21: lsl s_9_20 s_9_11
        let s_9_21: Bits = s_9_20 << s_9_11;
        // C s_9_22: lsl s_9_19 s_9_11
        let s_9_22: Bits = s_9_19 << s_9_11;
        // C s_9_23: cmpl s_9_22
        let s_9_23: Bits = !s_9_22;
        // D s_9_24: and s_9_13 s_9_23
        let s_9_24: Bits = ((s_9_13) & (s_9_23));
        // D s_9_25: or s_9_24 s_9_21
        let s_9_25: Bits = ((s_9_24) | (s_9_21));
        // D s_9_26: cast reint s_9_25 -> u32
        let s_9_26: u32 = (s_9_25.value() as u32);
        // D s_9_27: write-var target <= s_9_26
        fn_state.target = s_9_26;
        // C s_9_28: const #0s : i
        let s_9_28: i128 = 0;
        // D s_9_29: cast zx s_9_2 -> bv
        let s_9_29: Bits = Bits::new(s_9_2 as u128, 5u16);
        // C s_9_30: const #1s : i64
        let s_9_30: i64 = 1;
        // C s_9_31: cast zx s_9_30 -> i
        let s_9_31: i128 = (i128::try_from(s_9_30).unwrap());
        // C s_9_32: const #3s : i
        let s_9_32: i128 = 3;
        // C s_9_33: add s_9_32 s_9_31
        let s_9_33: i128 = (s_9_32 + s_9_31);
        // D s_9_34: bit-extract s_9_29 s_9_28 s_9_33
        let s_9_34: Bits = (Bits::new(
            ((s_9_29) >> (s_9_28)).value(),
            u16::try_from(s_9_33).unwrap(),
        ));
        // D s_9_35: cast reint s_9_34 -> u8
        let s_9_35: u8 = (s_9_34.value() as u8);
        // C s_9_36: const #0s : i
        let s_9_36: i128 = 0;
        // D s_9_37: read-var target:u32
        let s_9_37: u32 = fn_state.target;
        // D s_9_38: cast zx s_9_37 -> bv
        let s_9_38: Bits = Bits::new(s_9_37 as u128, 32u16);
        // D s_9_39: cast zx s_9_35 -> bv
        let s_9_39: Bits = Bits::new(s_9_35 as u128, 4u16);
        // C s_9_40: const #3s : i
        let s_9_40: i128 = 3;
        // C s_9_41: const #1u : u64
        let s_9_41: u64 = 1;
        // C s_9_42: cast zx s_9_41 -> bv
        let s_9_42: Bits = Bits::new(s_9_41 as u128, 64u16);
        // C s_9_43: lsl s_9_42 s_9_40
        let s_9_43: Bits = s_9_42 << s_9_40;
        // C s_9_44: sub s_9_43 s_9_42
        let s_9_44: Bits = ((s_9_43) - (s_9_42));
        // D s_9_45: and s_9_39 s_9_44
        let s_9_45: Bits = ((s_9_39) & (s_9_44));
        // D s_9_46: lsl s_9_45 s_9_36
        let s_9_46: Bits = s_9_45 << s_9_36;
        // C s_9_47: lsl s_9_44 s_9_36
        let s_9_47: Bits = s_9_44 << s_9_36;
        // C s_9_48: cmpl s_9_47
        let s_9_48: Bits = !s_9_47;
        // D s_9_49: and s_9_38 s_9_48
        let s_9_49: Bits = ((s_9_38) & (s_9_48));
        // D s_9_50: or s_9_49 s_9_46
        let s_9_50: Bits = ((s_9_49) | (s_9_46));
        // D s_9_51: cast reint s_9_50 -> u32
        let s_9_51: u32 = (s_9_50.value() as u32);
        // D s_9_52: write-var target <= s_9_51
        fn_state.target = s_9_51;
        // N s_9_53: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_10_0: read-var target:u32
        let s_10_0: u32 = fn_state.target;
        // N s_10_1: return s_10_0
        return s_10_0;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_11_0: read-var fault.16:struct
        let s_11_0: u32 = fn_state.fault._16;
        // D s_11_1: read-var fault.9:struct
        let s_11_1: i128 = fn_state.fault._9;
        // D s_11_2: call EncodeLDFSC(s_11_0, s_11_1)
        let s_11_2: u8 = EncodeLDFSC(state, tracer, s_11_0, s_11_1);
        // C s_11_3: const #0s : i
        let s_11_3: i128 = 0;
        // D s_11_4: read-var target:u32
        let s_11_4: u32 = fn_state.target;
        // D s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 32u16);
        // D s_11_6: cast zx s_11_2 -> bv
        let s_11_6: Bits = Bits::new(s_11_2 as u128, 6u16);
        // C s_11_7: const #5s : i
        let s_11_7: i128 = 5;
        // C s_11_8: const #1u : u64
        let s_11_8: u64 = 1;
        // C s_11_9: cast zx s_11_8 -> bv
        let s_11_9: Bits = Bits::new(s_11_8 as u128, 64u16);
        // C s_11_10: lsl s_11_9 s_11_7
        let s_11_10: Bits = s_11_9 << s_11_7;
        // C s_11_11: sub s_11_10 s_11_9
        let s_11_11: Bits = ((s_11_10) - (s_11_9));
        // D s_11_12: and s_11_6 s_11_11
        let s_11_12: Bits = ((s_11_6) & (s_11_11));
        // D s_11_13: lsl s_11_12 s_11_3
        let s_11_13: Bits = s_11_12 << s_11_3;
        // C s_11_14: lsl s_11_11 s_11_3
        let s_11_14: Bits = s_11_11 << s_11_3;
        // C s_11_15: cmpl s_11_14
        let s_11_15: Bits = !s_11_14;
        // D s_11_16: and s_11_5 s_11_15
        let s_11_16: Bits = ((s_11_5) & (s_11_15));
        // D s_11_17: or s_11_16 s_11_13
        let s_11_17: Bits = ((s_11_16) | (s_11_13));
        // D s_11_18: cast reint s_11_17 -> u32
        let s_11_18: u32 = (s_11_17.value() as u32);
        // D s_11_19: write-var target <= s_11_18
        fn_state.target = s_11_18;
        // N s_11_20: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var ga#6428 <= s_12_0
        fn_state.ga_6428 = s_12_0;
        // N s_12_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_13_0: read-var fault.5:struct
        let s_13_0: bool = fn_state.fault._5;
        // D s_13_1: call Bit(s_13_0)
        let s_13_1: bool = Bit(state, tracer, s_13_0);
        // C s_13_2: const #12s : i
        let s_13_2: i128 = 12;
        // D s_13_3: read-var target:u32
        let s_13_3: u32 = fn_state.target;
        // D s_13_4: cast zx s_13_3 -> bv
        let s_13_4: Bits = Bits::new(s_13_3 as u128, 32u16);
        // C s_13_5: const #1u : u64
        let s_13_5: u64 = 1;
        // D s_13_6: bit-insert s_13_4 s_13_4 s_13_2 s_13_5
        let s_13_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_13_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_13_4.length(),
            );
            (s_13_4 & mask) | (s_13_4 << s_13_2)
        };
        // D s_13_7: cast reint s_13_6 -> u32
        let s_13_7: u32 = (s_13_6.value() as u32);
        // D s_13_8: write-var target <= s_13_7
        fn_state.target = s_13_7;
        // N s_13_9: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_14_0: read-var fault:struct
        let s_14_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_14_1: call AArch32_PEErrorState(s_14_0)
        let s_14_1: u32 = AArch32_PEErrorState(state, tracer, s_14_0);
        // D s_14_2: call AArch32_EncodeAsyncErrorSyndrome(s_14_1)
        let s_14_2: u8 = AArch32_EncodeAsyncErrorSyndrome(state, tracer, s_14_1);
        // C s_14_3: const #14s : i
        let s_14_3: i128 = 14;
        // D s_14_4: read-var target:u32
        let s_14_4: u32 = fn_state.target;
        // D s_14_5: cast zx s_14_4 -> bv
        let s_14_5: Bits = Bits::new(s_14_4 as u128, 32u16);
        // D s_14_6: cast zx s_14_2 -> bv
        let s_14_6: Bits = Bits::new(s_14_2 as u128, 2u16);
        // C s_14_7: const #1s : i
        let s_14_7: i128 = 1;
        // C s_14_8: const #1u : u64
        let s_14_8: u64 = 1;
        // C s_14_9: cast zx s_14_8 -> bv
        let s_14_9: Bits = Bits::new(s_14_8 as u128, 64u16);
        // C s_14_10: lsl s_14_9 s_14_7
        let s_14_10: Bits = s_14_9 << s_14_7;
        // C s_14_11: sub s_14_10 s_14_9
        let s_14_11: Bits = ((s_14_10) - (s_14_9));
        // D s_14_12: and s_14_6 s_14_11
        let s_14_12: Bits = ((s_14_6) & (s_14_11));
        // D s_14_13: lsl s_14_12 s_14_3
        let s_14_13: Bits = s_14_12 << s_14_3;
        // C s_14_14: lsl s_14_11 s_14_3
        let s_14_14: Bits = s_14_11 << s_14_3;
        // C s_14_15: cmpl s_14_14
        let s_14_15: Bits = !s_14_14;
        // D s_14_16: and s_14_5 s_14_15
        let s_14_16: Bits = ((s_14_5) & (s_14_15));
        // D s_14_17: or s_14_16 s_14_13
        let s_14_17: Bits = ((s_14_16) | (s_14_13));
        // D s_14_18: cast reint s_14_17 -> u32
        let s_14_18: u32 = (s_14_17.value() as u32);
        // D s_14_19: write-var target <= s_14_18
        fn_state.target = s_14_18;
        // N s_14_20: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_15_0: read-var fault:struct
        let s_15_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_15_1: call IsAsyncAbort__1(s_15_0)
        let s_15_1: bool = IsAsyncAbort__1(state, tracer, s_15_0);
        // D s_15_2: write-var gs#9223 <= s_15_1
        fn_state.gs_9223 = s_15_1;
        // N s_15_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
