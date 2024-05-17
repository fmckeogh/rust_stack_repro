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
use Bit::*;
use set_subrange_zeros::*;
use Zeros::*;
use EncodeLDFSC::*;
use HaveRME::*;
use Unreachable::*;
use common::*;
pub fn DebugWriteFault<T: Tracer>(
    state: &mut State,
    tracer: &T,
    vaddress: u64,
    fault: ProductType1d757adad216cdef,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_20346: bool,
        ga_20320: ProductType9878976b5bcce9c9,
        ga_20343: bool,
        ga_20331: ProductType396b95aa74979079,
        ga_20328: ProductType396b95aa74979079,
        ga_20348: ProductType9878976b5bcce9c9,
        ec: u8,
        gs_25999: bool,
        syndrome: u64,
        gs_26000: bool,
        ga_20340: bool,
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
        // D s_0_0: read-var fault.0:struct
        let s_0_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_0_1: write-var ga#20320 <= s_0_0
        fn_state.ga_20320 = s_0_0;
        // D s_0_2: read-var ga#20320.1:struct
        let s_0_2: u32 = fn_state.ga_20320._1;
        // C s_0_3: const #10u : u32
        let s_0_3: u32 = 10;
        // D s_0_4: cmp-eq s_0_3 s_0_2
        let s_0_4: bool = ((s_0_3) == (s_0_2));
        // D s_0_5: not s_0_4
        let s_0_5: bool = !s_0_4;
        // N s_0_6: branch s_0_5 b29 b1
        if s_0_5 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #13704u : u32
        let s_1_0: u32 = 13704;
        // D s_1_1: read-reg s_1_0:u64
        let s_1_1: u64 = {
            let value = state.read_register::<u64>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // C s_1_2: const #0s : i
        let s_1_2: i128 = 0;
        // D s_1_3: cast zx s_1_1 -> bv
        let s_1_3: Bits = Bits::new(s_1_1 as u128, 64u16);
        // C s_1_4: const #1s : i64
        let s_1_4: i64 = 1;
        // C s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // C s_1_6: const #63s : i
        let s_1_6: i128 = 63;
        // C s_1_7: add s_1_6 s_1_5
        let s_1_7: i128 = (s_1_6 + s_1_5);
        // D s_1_8: bit-extract s_1_3 s_1_2 s_1_7
        let s_1_8: Bits = (Bits::new(
            ((s_1_3) >> (s_1_2)).value(),
            u16::try_from(s_1_7).unwrap(),
        ));
        // D s_1_9: cast reint s_1_8 -> u64
        let s_1_9: u64 = (s_1_8.value() as u64);
        // D s_1_10: write-var syndrome <= s_1_9
        fn_state.syndrome = s_1_9;
        // N s_1_11: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #10s : i
        let s_2_0: i128 = 10;
        // S s_2_1: call Zeros(s_2_0)
        let s_2_1: Bits = Zeros(state, tracer, s_2_0);
        // S s_2_2: cast reint s_2_1 -> u10
        let s_2_2: u16 = (s_2_1.value() as u16);
        // D s_2_3: read-var fault.16:struct
        let s_2_3: u32 = fn_state.fault._16;
        // D s_2_4: read-var fault.9:struct
        let s_2_4: i128 = fn_state.fault._9;
        // D s_2_5: call EncodeLDFSC(s_2_3, s_2_4)
        let s_2_5: u8 = EncodeLDFSC(state, tracer, s_2_3, s_2_4);
        // S s_2_6: cast zx s_2_2 -> bv
        let s_2_6: Bits = Bits::new(s_2_2 as u128, 10u16);
        // D s_2_7: cast zx s_2_5 -> bv
        let s_2_7: Bits = Bits::new(s_2_5 as u128, 6u16);
        // S s_2_8: cast reint s_2_6 -> u128
        let s_2_8: u128 = (s_2_6.value() as u128);
        // D s_2_9: size-of s_2_6
        let s_2_9: u16 = s_2_6.length();
        // D s_2_10: cast reint s_2_7 -> u128
        let s_2_10: u128 = (s_2_7.value() as u128);
        // D s_2_11: size-of s_2_7
        let s_2_11: u16 = s_2_7.length();
        // D s_2_12: lsl s_2_8 s_2_11
        let s_2_12: u128 = s_2_8 << s_2_11;
        // D s_2_13: or s_2_12 s_2_10
        let s_2_13: u128 = ((s_2_12) | (s_2_10));
        // D s_2_14: add s_2_9 s_2_11
        let s_2_14: u16 = (s_2_9 + s_2_11);
        // D s_2_15: create-bits s_2_13 s_2_14
        let s_2_15: Bits = Bits::new(s_2_13, s_2_14);
        // D s_2_16: cast reint s_2_15 -> u16
        let s_2_16: u16 = (s_2_15.value() as u16);
        // C s_2_17: const #0s : i
        let s_2_17: i128 = 0;
        // D s_2_18: read-var syndrome:u64
        let s_2_18: u64 = fn_state.syndrome;
        // D s_2_19: cast zx s_2_18 -> bv
        let s_2_19: Bits = Bits::new(s_2_18 as u128, 64u16);
        // D s_2_20: cast zx s_2_16 -> bv
        let s_2_20: Bits = Bits::new(s_2_16 as u128, 16u16);
        // C s_2_21: const #15s : i
        let s_2_21: i128 = 15;
        // C s_2_22: const #1u : u64
        let s_2_22: u64 = 1;
        // C s_2_23: cast zx s_2_22 -> bv
        let s_2_23: Bits = Bits::new(s_2_22 as u128, 64u16);
        // C s_2_24: lsl s_2_23 s_2_21
        let s_2_24: Bits = s_2_23 << s_2_21;
        // C s_2_25: sub s_2_24 s_2_23
        let s_2_25: Bits = ((s_2_24) - (s_2_23));
        // D s_2_26: and s_2_20 s_2_25
        let s_2_26: Bits = ((s_2_20) & (s_2_25));
        // D s_2_27: lsl s_2_26 s_2_17
        let s_2_27: Bits = s_2_26 << s_2_17;
        // C s_2_28: lsl s_2_25 s_2_17
        let s_2_28: Bits = s_2_25 << s_2_17;
        // C s_2_29: cmpl s_2_28
        let s_2_29: Bits = !s_2_28;
        // D s_2_30: and s_2_19 s_2_29
        let s_2_30: Bits = ((s_2_19) & (s_2_29));
        // D s_2_31: or s_2_30 s_2_27
        let s_2_31: Bits = ((s_2_30) | (s_2_27));
        // D s_2_32: cast reint s_2_31 -> u64
        let s_2_32: u64 = (s_2_31.value() as u64);
        // D s_2_33: write-var syndrome <= s_2_32
        fn_state.syndrome = s_2_32;
        // C s_2_34: const #64s : i
        let s_2_34: i128 = 64;
        // C s_2_35: const #55s : i
        let s_2_35: i128 = 55;
        // C s_2_36: const #32s : i
        let s_2_36: i128 = 32;
        // D s_2_37: read-var syndrome:u64
        let s_2_37: u64 = fn_state.syndrome;
        // D s_2_38: cast zx s_2_37 -> bv
        let s_2_38: Bits = Bits::new(s_2_37 as u128, 64u16);
        // D s_2_39: call set_subrange_zeros(s_2_34, s_2_38, s_2_35, s_2_36)
        let s_2_39: Bits = set_subrange_zeros(
            state,
            tracer,
            s_2_34,
            s_2_38,
            s_2_35,
            s_2_36,
        );
        // D s_2_40: cast reint s_2_39 -> u64
        let s_2_40: u64 = (s_2_39.value() as u64);
        // D s_2_41: write-var syndrome <= s_2_40
        fn_state.syndrome = s_2_40;
        // C s_2_42: const #() : ()
        let s_2_42: () = ();
        // S s_2_43: call HaveRME(s_2_42)
        let s_2_43: bool = HaveRME(state, tracer, s_2_42);
        // N s_2_44: branch s_2_43 b28 b3
        if s_2_43 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#25999 <= s_3_0
        fn_state.gs_25999 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#25999:u8
        let s_4_0: bool = fn_state.gs_25999;
        // N s_4_1: branch s_4_0 b27 b5
        if s_4_0 {
            return block_27(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#26000 <= s_5_0
        fn_state.gs_26000 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#26000:u8
        let s_6_0: bool = fn_state.gs_26000;
        // N s_6_1: branch s_6_0 b26 b7
        if s_6_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var fault.15:struct
        let s_7_0: bool = fn_state.fault._15;
        // N s_7_1: branch s_7_0 b25 b8
        if s_7_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #36u : u8
        let s_8_0: u8 = 36;
        // D s_8_1: write-var ec <= s_8_0
        fn_state.ec = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #26s : i
        let s_10_0: i128 = 26;
        // D s_10_1: read-var syndrome:u64
        let s_10_1: u64 = fn_state.syndrome;
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 64u16);
        // D s_10_3: read-var ec:u8
        let s_10_3: u8 = fn_state.ec;
        // D s_10_4: cast zx s_10_3 -> bv
        let s_10_4: Bits = Bits::new(s_10_3 as u128, 6u16);
        // C s_10_5: const #5s : i
        let s_10_5: i128 = 5;
        // C s_10_6: const #1u : u64
        let s_10_6: u64 = 1;
        // C s_10_7: cast zx s_10_6 -> bv
        let s_10_7: Bits = Bits::new(s_10_6 as u128, 64u16);
        // C s_10_8: lsl s_10_7 s_10_5
        let s_10_8: Bits = s_10_7 << s_10_5;
        // C s_10_9: sub s_10_8 s_10_7
        let s_10_9: Bits = ((s_10_8) - (s_10_7));
        // D s_10_10: and s_10_4 s_10_9
        let s_10_10: Bits = ((s_10_4) & (s_10_9));
        // D s_10_11: lsl s_10_10 s_10_0
        let s_10_11: Bits = s_10_10 << s_10_0;
        // C s_10_12: lsl s_10_9 s_10_0
        let s_10_12: Bits = s_10_9 << s_10_0;
        // C s_10_13: cmpl s_10_12
        let s_10_13: Bits = !s_10_12;
        // D s_10_14: and s_10_2 s_10_13
        let s_10_14: Bits = ((s_10_2) & (s_10_13));
        // D s_10_15: or s_10_14 s_10_11
        let s_10_15: Bits = ((s_10_14) | (s_10_11));
        // D s_10_16: cast reint s_10_15 -> u64
        let s_10_16: u64 = (s_10_15.value() as u64);
        // D s_10_17: write-var syndrome <= s_10_16
        fn_state.syndrome = s_10_16;
        // C s_10_18: const #1u : u8
        let s_10_18: bool = true;
        // S s_10_19: call Bit(s_10_18)
        let s_10_19: bool = Bit(state, tracer, s_10_18);
        // C s_10_20: const #17s : i
        let s_10_20: i128 = 17;
        // D s_10_21: read-var syndrome:u64
        let s_10_21: u64 = fn_state.syndrome;
        // D s_10_22: cast zx s_10_21 -> bv
        let s_10_22: Bits = Bits::new(s_10_21 as u128, 64u16);
        // C s_10_23: const #1u : u64
        let s_10_23: u64 = 1;
        // D s_10_24: bit-insert s_10_22 s_10_22 s_10_20 s_10_23
        let s_10_24: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_10_23 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_10_22.length(),
            );
            (s_10_22 & mask) | (s_10_22 << s_10_20)
        };
        // D s_10_25: cast reint s_10_24 -> u64
        let s_10_25: u64 = (s_10_24.value() as u64);
        // D s_10_26: write-var syndrome <= s_10_25
        fn_state.syndrome = s_10_25;
        // D s_10_27: read-var fault.16:struct
        let s_10_27: u32 = fn_state.fault._16;
        // C s_10_28: const #5u : u32
        let s_10_28: u32 = 5;
        // D s_10_29: cmp-eq s_10_27 s_10_28
        let s_10_29: bool = ((s_10_27) == (s_10_28));
        // N s_10_30: branch s_10_29 b15 b11
        if s_10_29 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var fault.0:struct
        let s_12_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_12_1: write-var ga#20348 <= s_12_0
        fn_state.ga_20348 = s_12_0;
        // D s_12_2: read-var ga#20348.1:struct
        let s_12_2: u32 = fn_state.ga_20348._1;
        // C s_12_3: const #10u : u32
        let s_12_3: u32 = 10;
        // D s_12_4: cmp-eq s_12_3 s_12_2
        let s_12_4: bool = ((s_12_3) == (s_12_2));
        // D s_12_5: not s_12_4
        let s_12_5: bool = !s_12_4;
        // N s_12_6: branch s_12_5 b14 b13
        if s_12_5 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #13704u : u32
        let s_13_0: u32 = 13704;
        // D s_13_1: read-reg s_13_0:struct
        let s_13_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // C s_13_2: const #13704u : u32
        let s_13_2: u32 = 13704;
        // N s_13_3: write-reg s_13_2 <= s_13_1
        let s_13_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_13_2 as isize, s_13_1);
            tracer.write_register(s_13_2 as isize, s_13_1);
        };
        // N s_13_4: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call Unreachable(s_14_0)
        let s_14_1: () = Unreachable(state, tracer, s_14_0);
        // N s_14_2: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var fault.1:struct
        let s_15_0: bool = fn_state.fault._1;
        // N s_15_1: branch s_15_0 b24 b16
        if s_15_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var ga#20340 <= s_16_0
        fn_state.ga_20340 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var ga#20340:u8
        let s_17_0: bool = fn_state.ga_20340;
        // D s_17_1: call Bit(s_17_0)
        let s_17_1: bool = Bit(state, tracer, s_17_0);
        // C s_17_2: const #39s : i
        let s_17_2: i128 = 39;
        // D s_17_3: read-var syndrome:u64
        let s_17_3: u64 = fn_state.syndrome;
        // D s_17_4: cast zx s_17_3 -> bv
        let s_17_4: Bits = Bits::new(s_17_3 as u128, 64u16);
        // C s_17_5: const #1u : u64
        let s_17_5: u64 = 1;
        // D s_17_6: bit-insert s_17_4 s_17_4 s_17_2 s_17_5
        let s_17_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_17_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_17_4.length(),
            );
            (s_17_4 & mask) | (s_17_4 << s_17_2)
        };
        // D s_17_7: cast reint s_17_6 -> u64
        let s_17_7: u64 = (s_17_6.value() as u64);
        // D s_17_8: write-var syndrome <= s_17_7
        fn_state.syndrome = s_17_7;
        // D s_17_9: read-var fault.11:struct
        let s_17_9: bool = fn_state.fault._11;
        // N s_17_10: branch s_17_9 b23 b18
        if s_17_9 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var ga#20343 <= s_18_0
        fn_state.ga_20343 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var ga#20343:u8
        let s_19_0: bool = fn_state.ga_20343;
        // D s_19_1: call Bit(s_19_0)
        let s_19_1: bool = Bit(state, tracer, s_19_0);
        // C s_19_2: const #38s : i
        let s_19_2: i128 = 38;
        // D s_19_3: read-var syndrome:u64
        let s_19_3: u64 = fn_state.syndrome;
        // D s_19_4: cast zx s_19_3 -> bv
        let s_19_4: Bits = Bits::new(s_19_3 as u128, 64u16);
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
        // D s_19_7: cast reint s_19_6 -> u64
        let s_19_7: u64 = (s_19_6.value() as u64);
        // D s_19_8: write-var syndrome <= s_19_7
        fn_state.syndrome = s_19_7;
        // D s_19_9: read-var fault.3:struct
        let s_19_9: bool = fn_state.fault._3;
        // N s_19_10: branch s_19_9 b22 b20
        if s_19_9 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var ga#20346 <= s_20_0
        fn_state.ga_20346 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var ga#20346:u8
        let s_21_0: bool = fn_state.ga_20346;
        // D s_21_1: call Bit(s_21_0)
        let s_21_1: bool = Bit(state, tracer, s_21_0);
        // C s_21_2: const #37s : i
        let s_21_2: i128 = 37;
        // D s_21_3: read-var syndrome:u64
        let s_21_3: u64 = fn_state.syndrome;
        // D s_21_4: cast zx s_21_3 -> bv
        let s_21_4: Bits = Bits::new(s_21_3 as u128, 64u16);
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
        // D s_21_7: cast reint s_21_6 -> u64
        let s_21_7: u64 = (s_21_6.value() as u64);
        // D s_21_8: write-var syndrome <= s_21_7
        fn_state.syndrome = s_21_7;
        // N s_21_9: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var ga#20346 <= s_22_0
        fn_state.ga_20346 = s_22_0;
        // N s_22_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var ga#20343 <= s_23_0
        fn_state.ga_20343 = s_23_0;
        // N s_23_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var ga#20340 <= s_24_0
        fn_state.ga_20340 = s_24_0;
        // N s_24_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #37u : u8
        let s_25_0: u8 = 37;
        // D s_25_1: write-var ec <= s_25_0
        fn_state.ec = s_25_0;
        // N s_25_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #30u : u8
        let s_26_0: u8 = 30;
        // D s_26_1: write-var ec <= s_26_0
        fn_state.ec = s_26_0;
        // N s_26_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var fault.6:struct
        let s_27_0: ProductType396b95aa74979079 = fn_state.fault._6;
        // D s_27_1: write-var ga#20331 <= s_27_0
        fn_state.ga_20331 = s_27_0;
        // D s_27_2: read-var ga#20331.0:struct
        let s_27_2: u32 = fn_state.ga_20331._0;
        // C s_27_3: const #4u : u32
        let s_27_3: u32 = 4;
        // D s_27_4: cmp-eq s_27_2 s_27_3
        let s_27_4: bool = ((s_27_2) == (s_27_3));
        // D s_27_5: write-var gs#26000 <= s_27_4
        fn_state.gs_26000 = s_27_4;
        // N s_27_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var fault.6:struct
        let s_28_0: ProductType396b95aa74979079 = fn_state.fault._6;
        // D s_28_1: write-var ga#20328 <= s_28_0
        fn_state.ga_20328 = s_28_0;
        // D s_28_2: read-var ga#20328.0:struct
        let s_28_2: u32 = fn_state.ga_20328._0;
        // C s_28_3: const #0u : u32
        let s_28_3: u32 = 0;
        // D s_28_4: cmp-eq s_28_2 s_28_3
        let s_28_4: bool = ((s_28_2) == (s_28_3));
        // D s_28_5: write-var gs#25999 <= s_28_4
        fn_state.gs_25999 = s_28_4;
        // N s_28_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call Unreachable(s_29_0)
        let s_29_1: () = Unreachable(state, tracer, s_29_0);
        // N s_29_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
