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
use u__UNKNOWN_bits::*;
use HPFAR_write::*;
use HPFAR_read::*;
use HIFAR_write::*;
use integer_subrange::*;
use AArch32_ExceptionClass::*;
use HSR_write::*;
use HDFAR_write::*;
use Mk_HPFAR_Type::*;
use Mk_HSR_Type::*;
use common::*;
pub fn AArch32_ReportHypEntry<T: Tracer>(
    state: &mut State,
    tracer: &T,
    except: ProductTypeb7f99f96751e17c4,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        iss: u32,
        ga_6213: ProductType700c18a878c5601b,
        gs_8888: bool,
        gs_8885: bool,
        gs_8892: bool,
        ecshadow_124: i128,
        ga_6189: ProductTypec1bd230b943b3b8c,
        ga_6219: ProductType700c18a878c5601b,
        il: bool,
        exceptype: u32,
        except: ProductTypeb7f99f96751e17c4,
    }
    let fn_state = FunctionState {
        except,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var except.1:struct
        let s_0_0: u32 = fn_state.except._1;
        // D s_0_1: write-var exceptype <= s_0_0
        fn_state.exceptype = s_0_0;
        // D s_0_2: read-var exceptype:u32
        let s_0_2: u32 = fn_state.exceptype;
        // D s_0_3: call AArch32_ExceptionClass(s_0_2)
        let s_0_3: ProductTypec1bd230b943b3b8c = AArch32_ExceptionClass(
            state,
            tracer,
            s_0_2,
        );
        // D s_0_4: write-var ga#6189 <= s_0_3
        fn_state.ga_6189 = s_0_3;
        // D s_0_5: read-var ga#6189.0:struct
        let s_0_5: i128 = fn_state.ga_6189._0;
        // D s_0_6: read-var ga#6189.1:struct
        let s_0_6: bool = fn_state.ga_6189._1;
        // D s_0_7: write-var il <= s_0_6
        fn_state.il = s_0_6;
        // D s_0_8: write-var ecshadow#124 <= s_0_5
        fn_state.ecshadow_124 = s_0_5;
        // D s_0_9: read-var except.6:struct
        let s_0_9: u32 = fn_state.except._6;
        // D s_0_10: write-var iss <= s_0_9
        fn_state.iss = s_0_9;
        // C s_0_11: const #36u : u8
        let s_0_11: u8 = 36;
        // C s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 8u16);
        // C s_0_13: cast zx s_0_12 -> i
        let s_0_13: i128 = (s_0_12.value() as i128);
        // C s_0_14: cast reint s_0_13 -> i64
        let s_0_14: i64 = (s_0_13 as i64);
        // C s_0_15: cast zx s_0_14 -> i
        let s_0_15: i128 = (i128::try_from(s_0_14).unwrap());
        // D s_0_16: read-var ecshadow#124:i
        let s_0_16: i128 = fn_state.ecshadow_124;
        // D s_0_17: cmp-eq s_0_16 s_0_15
        let s_0_17: bool = ((s_0_16) == (s_0_15));
        // N s_0_18: branch s_0_17 b19 b1
        if s_0_17 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #37u : u8
        let s_1_0: u8 = 37;
        // C s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 8u16);
        // C s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // C s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // C s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: read-var ecshadow#124:i
        let s_1_5: i128 = fn_state.ecshadow_124;
        // D s_1_6: cmp-eq s_1_5 s_1_4
        let s_1_6: bool = ((s_1_5) == (s_1_4));
        // D s_1_7: write-var gs#8885 <= s_1_6
        fn_state.gs_8885 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#8885:u8
        let s_2_0: bool = fn_state.gs_8885;
        // N s_2_1: branch s_2_0 b18 b3
        if s_2_0 {
            return block_18(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#8888 <= s_3_0
        fn_state.gs_8888 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#8888:u8
        let s_4_0: bool = fn_state.gs_8888;
        // N s_4_1: branch s_4_0 b17 b5
        if s_4_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var il:u8
        let s_6_0: bool = fn_state.il;
        // C s_6_1: const #5s : i
        let s_6_1: i128 = 5;
        // C s_6_2: const #0s : i
        let s_6_2: i128 = 0;
        // D s_6_3: read-var ecshadow#124:i
        let s_6_3: i128 = fn_state.ecshadow_124;
        // D s_6_4: call integer_subrange(s_6_3, s_6_1, s_6_2)
        let s_6_4: Bits = integer_subrange(state, tracer, s_6_3, s_6_1, s_6_2);
        // D s_6_5: cast reint s_6_4 -> u8
        let s_6_5: u8 = (s_6_4.value() as u8);
        // D s_6_6: cast zx s_6_5 -> bv
        let s_6_6: Bits = Bits::new(s_6_5 as u128, 6u16);
        // D s_6_7: cast zx s_6_0 -> bv
        let s_6_7: Bits = Bits::new(s_6_0 as u128, 1u16);
        // D s_6_8: cast reint s_6_6 -> u128
        let s_6_8: u128 = (s_6_6.value() as u128);
        // D s_6_9: size-of s_6_6
        let s_6_9: u16 = s_6_6.length();
        // D s_6_10: cast reint s_6_7 -> u128
        let s_6_10: u128 = (s_6_7.value() as u128);
        // D s_6_11: size-of s_6_7
        let s_6_11: u16 = s_6_7.length();
        // D s_6_12: lsl s_6_8 s_6_11
        let s_6_12: u128 = s_6_8 << s_6_11;
        // D s_6_13: or s_6_12 s_6_10
        let s_6_13: u128 = ((s_6_12) | (s_6_10));
        // D s_6_14: add s_6_9 s_6_11
        let s_6_14: u16 = (s_6_9 + s_6_11);
        // D s_6_15: create-bits s_6_13 s_6_14
        let s_6_15: Bits = Bits::new(s_6_13, s_6_14);
        // D s_6_16: cast reint s_6_15 -> u8
        let s_6_16: u8 = (s_6_15.value() as u8);
        // D s_6_17: cast zx s_6_16 -> bv
        let s_6_17: Bits = Bits::new(s_6_16 as u128, 7u16);
        // D s_6_18: read-var iss:u25
        let s_6_18: u32 = fn_state.iss;
        // D s_6_19: cast zx s_6_18 -> bv
        let s_6_19: Bits = Bits::new(s_6_18 as u128, 25u16);
        // D s_6_20: cast reint s_6_17 -> u128
        let s_6_20: u128 = (s_6_17.value() as u128);
        // D s_6_21: size-of s_6_17
        let s_6_21: u16 = s_6_17.length();
        // D s_6_22: cast reint s_6_19 -> u128
        let s_6_22: u128 = (s_6_19.value() as u128);
        // D s_6_23: size-of s_6_19
        let s_6_23: u16 = s_6_19.length();
        // D s_6_24: lsl s_6_20 s_6_23
        let s_6_24: u128 = s_6_20 << s_6_23;
        // D s_6_25: or s_6_24 s_6_22
        let s_6_25: u128 = ((s_6_24) | (s_6_22));
        // D s_6_26: add s_6_21 s_6_23
        let s_6_26: u16 = (s_6_21 + s_6_23);
        // D s_6_27: create-bits s_6_25 s_6_26
        let s_6_27: Bits = Bits::new(s_6_25, s_6_26);
        // D s_6_28: cast reint s_6_27 -> u32
        let s_6_28: u32 = (s_6_27.value() as u32);
        // D s_6_29: call Mk_HSR_Type(s_6_28)
        let s_6_29: ProductType700c18a878c5601b = Mk_HSR_Type(state, tracer, s_6_28);
        // D s_6_30: call HSR_write(s_6_29)
        let s_6_30: () = HSR_write(state, tracer, s_6_29);
        // D s_6_31: read-var exceptype:u32
        let s_6_31: u32 = fn_state.exceptype;
        // C s_6_32: const #17u : u32
        let s_6_32: u32 = 17;
        // D s_6_33: cmp-eq s_6_31 s_6_32
        let s_6_33: bool = ((s_6_31) == (s_6_32));
        // N s_6_34: branch s_6_33 b16 b7
        if s_6_33 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var exceptype:u32
        let s_7_0: u32 = fn_state.exceptype;
        // C s_7_1: const #18u : u32
        let s_7_1: u32 = 18;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // D s_7_3: write-var gs#8892 <= s_7_2
        fn_state.gs_8892 = s_7_2;
        // N s_7_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#8892:u8
        let s_8_0: bool = fn_state.gs_8892;
        // N s_8_1: branch s_8_0 b15 b9
        if s_8_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var exceptype:u32
        let s_9_0: u32 = fn_state.exceptype;
        // C s_9_1: const #19u : u32
        let s_9_1: u32 = 19;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // N s_9_3: branch s_9_2 b14 b10
        if s_9_2 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var except.3:struct
        let s_11_0: bool = fn_state.except._3;
        // N s_11_1: branch s_11_0 b13 b12
        if s_11_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call HPFAR_read(s_12_0)
        let s_12_1: ProductType700c18a878c5601b = HPFAR_read(state, tracer, s_12_0);
        // D s_12_2: write-var ga#6219 <= s_12_1
        fn_state.ga_6219 = s_12_1;
        // D s_12_3: read-var ga#6219.0:struct
        let s_12_3: u32 = fn_state.ga_6219._0;
        // C s_12_4: const #28s : i64
        let s_12_4: i64 = 28;
        // C s_12_5: cast zx s_12_4 -> i
        let s_12_5: i128 = (i128::try_from(s_12_4).unwrap());
        // S s_12_6: call __UNKNOWN_bits(s_12_5)
        let s_12_6: Bits = u__UNKNOWN_bits(state, tracer, s_12_5);
        // S s_12_7: cast reint s_12_6 -> u28
        let s_12_7: u32 = (s_12_6.value() as u32);
        // C s_12_8: const #4s : i
        let s_12_8: i128 = 4;
        // D s_12_9: cast zx s_12_3 -> bv
        let s_12_9: Bits = Bits::new(s_12_3 as u128, 32u16);
        // S s_12_10: cast zx s_12_7 -> bv
        let s_12_10: Bits = Bits::new(s_12_7 as u128, 28u16);
        // C s_12_11: const #27s : i
        let s_12_11: i128 = 27;
        // C s_12_12: const #1u : u64
        let s_12_12: u64 = 1;
        // C s_12_13: cast zx s_12_12 -> bv
        let s_12_13: Bits = Bits::new(s_12_12 as u128, 64u16);
        // C s_12_14: lsl s_12_13 s_12_11
        let s_12_14: Bits = s_12_13 << s_12_11;
        // C s_12_15: sub s_12_14 s_12_13
        let s_12_15: Bits = ((s_12_14) - (s_12_13));
        // S s_12_16: and s_12_10 s_12_15
        let s_12_16: Bits = ((s_12_10) & (s_12_15));
        // S s_12_17: lsl s_12_16 s_12_8
        let s_12_17: Bits = s_12_16 << s_12_8;
        // C s_12_18: lsl s_12_15 s_12_8
        let s_12_18: Bits = s_12_15 << s_12_8;
        // C s_12_19: cmpl s_12_18
        let s_12_19: Bits = !s_12_18;
        // D s_12_20: and s_12_9 s_12_19
        let s_12_20: Bits = ((s_12_9) & (s_12_19));
        // D s_12_21: or s_12_20 s_12_17
        let s_12_21: Bits = ((s_12_20) | (s_12_17));
        // D s_12_22: cast reint s_12_21 -> u32
        let s_12_22: u32 = (s_12_21.value() as u32);
        // D s_12_23: call Mk_HPFAR_Type(s_12_22)
        let s_12_23: ProductType700c18a878c5601b = Mk_HPFAR_Type(state, tracer, s_12_22);
        // D s_12_24: call HPFAR_write(s_12_23)
        let s_12_24: () = HPFAR_write(state, tracer, s_12_23);
        // N s_12_25: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call HPFAR_read(s_13_0)
        let s_13_1: ProductType700c18a878c5601b = HPFAR_read(state, tracer, s_13_0);
        // D s_13_2: write-var ga#6213 <= s_13_1
        fn_state.ga_6213 = s_13_1;
        // D s_13_3: read-var ga#6213.0:struct
        let s_13_3: u32 = fn_state.ga_6213._0;
        // D s_13_4: read-var except.2:struct
        let s_13_4: u64 = fn_state.except._2;
        // C s_13_5: const #12s : i
        let s_13_5: i128 = 12;
        // D s_13_6: cast zx s_13_4 -> bv
        let s_13_6: Bits = Bits::new(s_13_4 as u128, 56u16);
        // C s_13_7: const #1s : i64
        let s_13_7: i64 = 1;
        // C s_13_8: cast zx s_13_7 -> i
        let s_13_8: i128 = (i128::try_from(s_13_7).unwrap());
        // C s_13_9: const #27s : i
        let s_13_9: i128 = 27;
        // C s_13_10: add s_13_9 s_13_8
        let s_13_10: i128 = (s_13_9 + s_13_8);
        // D s_13_11: bit-extract s_13_6 s_13_5 s_13_10
        let s_13_11: Bits = (Bits::new(
            ((s_13_6) >> (s_13_5)).value(),
            u16::try_from(s_13_10).unwrap(),
        ));
        // D s_13_12: cast reint s_13_11 -> u28
        let s_13_12: u32 = (s_13_11.value() as u32);
        // C s_13_13: const #4s : i
        let s_13_13: i128 = 4;
        // D s_13_14: cast zx s_13_3 -> bv
        let s_13_14: Bits = Bits::new(s_13_3 as u128, 32u16);
        // D s_13_15: cast zx s_13_12 -> bv
        let s_13_15: Bits = Bits::new(s_13_12 as u128, 28u16);
        // C s_13_16: const #27s : i
        let s_13_16: i128 = 27;
        // C s_13_17: const #1u : u64
        let s_13_17: u64 = 1;
        // C s_13_18: cast zx s_13_17 -> bv
        let s_13_18: Bits = Bits::new(s_13_17 as u128, 64u16);
        // C s_13_19: lsl s_13_18 s_13_16
        let s_13_19: Bits = s_13_18 << s_13_16;
        // C s_13_20: sub s_13_19 s_13_18
        let s_13_20: Bits = ((s_13_19) - (s_13_18));
        // D s_13_21: and s_13_15 s_13_20
        let s_13_21: Bits = ((s_13_15) & (s_13_20));
        // D s_13_22: lsl s_13_21 s_13_13
        let s_13_22: Bits = s_13_21 << s_13_13;
        // C s_13_23: lsl s_13_20 s_13_13
        let s_13_23: Bits = s_13_20 << s_13_13;
        // C s_13_24: cmpl s_13_23
        let s_13_24: Bits = !s_13_23;
        // D s_13_25: and s_13_14 s_13_24
        let s_13_25: Bits = ((s_13_14) & (s_13_24));
        // D s_13_26: or s_13_25 s_13_22
        let s_13_26: Bits = ((s_13_25) | (s_13_22));
        // D s_13_27: cast reint s_13_26 -> u32
        let s_13_27: u32 = (s_13_26.value() as u32);
        // D s_13_28: call Mk_HPFAR_Type(s_13_27)
        let s_13_28: ProductType700c18a878c5601b = Mk_HPFAR_Type(state, tracer, s_13_27);
        // D s_13_29: call HPFAR_write(s_13_28)
        let s_13_29: () = HPFAR_write(state, tracer, s_13_28);
        // N s_13_30: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #32s : i64
        let s_14_0: i64 = 32;
        // C s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // S s_14_2: call __UNKNOWN_bits(s_14_1)
        let s_14_2: Bits = u__UNKNOWN_bits(state, tracer, s_14_1);
        // S s_14_3: cast reint s_14_2 -> u32
        let s_14_3: u32 = (s_14_2.value() as u32);
        // S s_14_4: call HIFAR_write(s_14_3)
        let s_14_4: () = HIFAR_write(state, tracer, s_14_3);
        // D s_14_5: read-var except.9:struct
        let s_14_5: u64 = fn_state.except._9;
        // C s_14_6: const #0s : i
        let s_14_6: i128 = 0;
        // D s_14_7: cast zx s_14_5 -> bv
        let s_14_7: Bits = Bits::new(s_14_5 as u128, 64u16);
        // C s_14_8: const #1s : i64
        let s_14_8: i64 = 1;
        // C s_14_9: cast zx s_14_8 -> i
        let s_14_9: i128 = (i128::try_from(s_14_8).unwrap());
        // C s_14_10: const #31s : i
        let s_14_10: i128 = 31;
        // C s_14_11: add s_14_10 s_14_9
        let s_14_11: i128 = (s_14_10 + s_14_9);
        // D s_14_12: bit-extract s_14_7 s_14_6 s_14_11
        let s_14_12: Bits = (Bits::new(
            ((s_14_7) >> (s_14_6)).value(),
            u16::try_from(s_14_11).unwrap(),
        ));
        // D s_14_13: cast reint s_14_12 -> u32
        let s_14_13: u32 = (s_14_12.value() as u32);
        // D s_14_14: call HDFAR_write(s_14_13)
        let s_14_14: () = HDFAR_write(state, tracer, s_14_13);
        // N s_14_15: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var except.9:struct
        let s_15_0: u64 = fn_state.except._9;
        // C s_15_1: const #0s : i
        let s_15_1: i128 = 0;
        // D s_15_2: cast zx s_15_0 -> bv
        let s_15_2: Bits = Bits::new(s_15_0 as u128, 64u16);
        // C s_15_3: const #1s : i64
        let s_15_3: i64 = 1;
        // C s_15_4: cast zx s_15_3 -> i
        let s_15_4: i128 = (i128::try_from(s_15_3).unwrap());
        // C s_15_5: const #31s : i
        let s_15_5: i128 = 31;
        // C s_15_6: add s_15_5 s_15_4
        let s_15_6: i128 = (s_15_5 + s_15_4);
        // D s_15_7: bit-extract s_15_2 s_15_1 s_15_6
        let s_15_7: Bits = (Bits::new(
            ((s_15_2) >> (s_15_1)).value(),
            u16::try_from(s_15_6).unwrap(),
        ));
        // D s_15_8: cast reint s_15_7 -> u32
        let s_15_8: u32 = (s_15_7.value() as u32);
        // D s_15_9: call HIFAR_write(s_15_8)
        let s_15_9: () = HIFAR_write(state, tracer, s_15_8);
        // C s_15_10: const #32s : i64
        let s_15_10: i64 = 32;
        // C s_15_11: cast zx s_15_10 -> i
        let s_15_11: i128 = (i128::try_from(s_15_10).unwrap());
        // S s_15_12: call __UNKNOWN_bits(s_15_11)
        let s_15_12: Bits = u__UNKNOWN_bits(state, tracer, s_15_11);
        // S s_15_13: cast reint s_15_12 -> u32
        let s_15_13: u32 = (s_15_12.value() as u32);
        // S s_15_14: call HDFAR_write(s_15_13)
        let s_15_14: () = HDFAR_write(state, tracer, s_15_13);
        // N s_15_15: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#8892 <= s_16_0
        fn_state.gs_8892 = s_16_0;
        // N s_16_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var il <= s_17_0
        fn_state.il = s_17_0;
        // N s_17_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #24s : i
        let s_18_0: i128 = 24;
        // D s_18_1: read-var iss:u25
        let s_18_1: u32 = fn_state.iss;
        // D s_18_2: cast zx s_18_1 -> bv
        let s_18_2: Bits = Bits::new(s_18_1 as u128, 25u16);
        // C s_18_3: const #1u : u64
        let s_18_3: u64 = 1;
        // D s_18_4: bit-extract s_18_2 s_18_0 s_18_3
        let s_18_4: Bits = (Bits::new(
            ((s_18_2) >> (s_18_0)).value(),
            u16::try_from(s_18_3).unwrap(),
        ));
        // D s_18_5: cast reint s_18_4 -> u8
        let s_18_5: bool = ((s_18_4.value()) != 0);
        // C s_18_6: const #0s : i
        let s_18_6: i128 = 0;
        // C s_18_7: const #0u : u64
        let s_18_7: u64 = 0;
        // D s_18_8: cast zx s_18_5 -> u64
        let s_18_8: u64 = (s_18_5 as u64);
        // C s_18_9: const #1u : u64
        let s_18_9: u64 = 1;
        // D s_18_10: and s_18_8 s_18_9
        let s_18_10: u64 = ((s_18_8) & (s_18_9));
        // D s_18_11: cmp-eq s_18_10 s_18_9
        let s_18_11: bool = ((s_18_10) == (s_18_9));
        // D s_18_12: lsl s_18_8 s_18_6
        let s_18_12: u64 = s_18_8 << s_18_6;
        // D s_18_13: or s_18_7 s_18_12
        let s_18_13: u64 = ((s_18_7) | (s_18_12));
        // D s_18_14: cmpl s_18_12
        let s_18_14: u64 = !s_18_12;
        // D s_18_15: and s_18_7 s_18_14
        let s_18_15: u64 = ((s_18_7) & (s_18_14));
        // D s_18_16: select s_18_11 s_18_13 s_18_15
        let s_18_16: u64 = if s_18_11 { s_18_13 } else { s_18_15 };
        // D s_18_17: cast trunc s_18_16 -> u8
        let s_18_17: bool = ((s_18_16) != 0);
        // D s_18_18: cast zx s_18_17 -> bv
        let s_18_18: Bits = Bits::new(s_18_17 as u128, 1u16);
        // C s_18_19: const #0u : u8
        let s_18_19: bool = false;
        // C s_18_20: cast zx s_18_19 -> bv
        let s_18_20: Bits = Bits::new(s_18_19 as u128, 1u16);
        // D s_18_21: cmp-eq s_18_18 s_18_20
        let s_18_21: bool = ((s_18_18) == (s_18_20));
        // D s_18_22: write-var gs#8888 <= s_18_21
        fn_state.gs_8888 = s_18_21;
        // N s_18_23: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#8885 <= s_19_0
        fn_state.gs_8885 = s_19_0;
        // N s_19_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
