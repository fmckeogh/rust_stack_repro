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
use Halt::*;
use SecurityStateAtEL::*;
use EDECCR_read::*;
use HaltingAllowed::*;
use Havev8p8Debug::*;
use HaveExtendedECDebugEvents::*;
use common::*;
pub fn CheckExceptionCatch<T: Tracer>(
    state: &mut State,
    tracer: &T,
    exception_entry: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_4155: ProductType700c18a878c5601b,
        base: i64,
        baseshadow_72: i64,
        halt: bool,
        ss: u32,
        ga_4139: i64,
        ga_4140: ProductType700c18a878c5601b,
        exception_exitshadow_73: bool,
        ctrlshadow_75: u8,
        gs_6320: bool,
        ga_4147: ProductType700c18a878c5601b,
        exception_entry: bool,
    }
    let fn_state = FunctionState {
        exception_entry,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call SecurityStateAtEL(s_0_1)
        let s_0_2: u32 = SecurityStateAtEL(state, tracer, s_0_1);
        // D s_0_3: write-var ss <= s_0_2
        fn_state.ss = s_0_2;
        // C s_0_4: const #0s : i64
        let s_0_4: i64 = 0;
        // D s_0_5: write-var base <= s_0_4
        fn_state.base = s_0_4;
        // C s_0_6: const #3u : u32
        let s_0_6: u32 = 3;
        // D s_0_7: read-var ss:u32
        let s_0_7: u32 = fn_state.ss;
        // D s_0_8: cmp-eq s_0_6 s_0_7
        let s_0_8: bool = ((s_0_6) == (s_0_7));
        // D s_0_9: not s_0_8
        let s_0_9: bool = !s_0_8;
        // N s_0_10: branch s_0_9 b25 b1
        if s_0_9 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0s : i64
        let s_1_0: i64 = 0;
        // D s_1_1: write-var base <= s_1_0
        fn_state.base = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var base:i64
        let s_2_0: i64 = fn_state.base;
        // D s_2_1: write-var baseshadow#72 <= s_2_0
        fn_state.baseshadow_72 = s_2_0;
        // C s_2_2: const #() : ()
        let s_2_2: () = ();
        // S s_2_3: call HaltingAllowed(s_2_2)
        let s_2_3: bool = HaltingAllowed(state, tracer, s_2_2);
        // N s_2_4: branch s_2_3 b4 b3
        if s_2_3 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call HaveExtendedECDebugEvents(s_4_0)
        let s_4_1: bool = HaveExtendedECDebugEvents(state, tracer, s_4_0);
        // N s_4_2: branch s_4_1 b14 b5
        if s_4_1 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call EDECCR_read(s_5_0)
        let s_5_1: ProductType700c18a878c5601b = EDECCR_read(state, tracer, s_5_0);
        // D s_5_2: write-var ga#4155 <= s_5_1
        fn_state.ga_4155 = s_5_1;
        // D s_5_3: read-var ga#4155.0:struct
        let s_5_3: u32 = fn_state.ga_4155._0;
        // C s_5_4: const #16975u : u32
        let s_5_4: u32 = 16975;
        // D s_5_5: read-reg s_5_4:u8
        let s_5_5: u8 = {
            let value = state.read_register::<u8>(s_5_4 as isize);
            tracer.read_register(s_5_4 as isize, value);
            value
        };
        // D s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 2u16);
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (s_5_6.value() as i128);
        // D s_5_8: cast reint s_5_7 -> i64
        let s_5_8: i64 = (s_5_7 as i64);
        // D s_5_9: cast zx s_5_8 -> i
        let s_5_9: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_10: read-var baseshadow#72:i64
        let s_5_10: i64 = fn_state.baseshadow_72;
        // D s_5_11: cast zx s_5_10 -> i
        let s_5_11: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_12: add s_5_9 s_5_11
        let s_5_12: i128 = (s_5_9 + s_5_11);
        // D s_5_13: cast reint s_5_12 -> i64
        let s_5_13: i64 = (s_5_12 as i64);
        // D s_5_14: cast zx s_5_3 -> bv
        let s_5_14: Bits = Bits::new(s_5_3 as u128, 32u16);
        // D s_5_15: cast zx s_5_13 -> i
        let s_5_15: i128 = (i128::try_from(s_5_13).unwrap());
        // C s_5_16: const #1u : u64
        let s_5_16: u64 = 1;
        // D s_5_17: bit-extract s_5_14 s_5_15 s_5_16
        let s_5_17: Bits = (Bits::new(
            ((s_5_14) >> (s_5_15)).value(),
            u16::try_from(s_5_16).unwrap(),
        ));
        // D s_5_18: cast reint s_5_17 -> u8
        let s_5_18: bool = ((s_5_17.value()) != 0);
        // C s_5_19: const #0s : i
        let s_5_19: i128 = 0;
        // C s_5_20: const #0u : u64
        let s_5_20: u64 = 0;
        // D s_5_21: cast zx s_5_18 -> u64
        let s_5_21: u64 = (s_5_18 as u64);
        // C s_5_22: const #1u : u64
        let s_5_22: u64 = 1;
        // D s_5_23: and s_5_21 s_5_22
        let s_5_23: u64 = ((s_5_21) & (s_5_22));
        // D s_5_24: cmp-eq s_5_23 s_5_22
        let s_5_24: bool = ((s_5_23) == (s_5_22));
        // D s_5_25: lsl s_5_21 s_5_19
        let s_5_25: u64 = s_5_21 << s_5_19;
        // D s_5_26: or s_5_20 s_5_25
        let s_5_26: u64 = ((s_5_20) | (s_5_25));
        // D s_5_27: cmpl s_5_25
        let s_5_27: u64 = !s_5_25;
        // D s_5_28: and s_5_20 s_5_27
        let s_5_28: u64 = ((s_5_20) & (s_5_27));
        // D s_5_29: select s_5_24 s_5_26 s_5_28
        let s_5_29: u64 = if s_5_24 { s_5_26 } else { s_5_28 };
        // D s_5_30: cast trunc s_5_29 -> u8
        let s_5_30: bool = ((s_5_29) != 0);
        // D s_5_31: cast zx s_5_30 -> bv
        let s_5_31: Bits = Bits::new(s_5_30 as u128, 1u16);
        // C s_5_32: const #1u : u8
        let s_5_32: bool = true;
        // C s_5_33: cast zx s_5_32 -> bv
        let s_5_33: Bits = Bits::new(s_5_32 as u128, 1u16);
        // D s_5_34: cmp-eq s_5_31 s_5_33
        let s_5_34: bool = ((s_5_31) == (s_5_33));
        // D s_5_35: write-var halt <= s_5_34
        fn_state.halt = s_5_34;
        // N s_5_36: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var halt:u8
        let s_6_0: bool = fn_state.halt;
        // N s_6_1: branch s_6_0 b8 b7
        if s_6_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call Havev8p8Debug(s_8_0)
        let s_8_1: bool = Havev8p8Debug(state, tracer, s_8_0);
        // N s_8_2: branch s_8_1 b13 b9
        if s_8_1 {
            return block_13(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#6320 <= s_9_0
        fn_state.gs_6320 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#6320:u8
        let s_10_0: bool = fn_state.gs_6320;
        // N s_10_1: branch s_10_0 b12 b11
        if s_10_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1168u : u32
        let s_11_0: u32 = 1168;
        // D s_11_1: read-reg s_11_0:u8
        let s_11_1: u8 = {
            let value = state.read_register::<u8>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: call Halt(s_11_1)
        let s_11_2: () = Halt(state, tracer, s_11_1);
        // N s_11_3: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #14712u : u32
        let s_12_0: u32 = 14712;
        // D s_12_1: read-reg s_12_0:struct
        let s_12_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // C s_12_2: const #14712u : u32
        let s_12_2: u32 = 14712;
        // N s_12_3: write-reg s_12_2 <= s_12_1
        let s_12_3: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_12_2 as isize, s_12_1);
            tracer.write_register(s_12_2 as isize, s_12_1);
        };
        // N s_12_4: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var exception_entry:u8
        let s_13_0: bool = fn_state.exception_entry;
        // D s_13_1: write-var gs#6320 <= s_13_0
        fn_state.gs_6320 = s_13_0;
        // N s_13_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var exception_entry:u8
        let s_14_0: bool = fn_state.exception_entry;
        // D s_14_1: not s_14_0
        let s_14_1: bool = !s_14_0;
        // D s_14_2: write-var exception_exitshadow#73 <= s_14_1
        fn_state.exception_exitshadow_73 = s_14_1;
        // D s_14_3: read-var ss:u32
        let s_14_3: u32 = fn_state.ss;
        // C s_14_4: const #2u : u32
        let s_14_4: u32 = 2;
        // D s_14_5: cmp-eq s_14_3 s_14_4
        let s_14_5: bool = ((s_14_3) == (s_14_4));
        // N s_14_6: branch s_14_5 b24 b15
        if s_14_5 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #8s : i64
        let s_15_0: i64 = 8;
        // D s_15_1: write-var ga#4139 <= s_15_0
        fn_state.ga_4139 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var ga#4139:i64
        let s_16_0: i64 = fn_state.ga_4139;
        // C s_16_1: const #() : ()
        let s_16_1: () = ();
        // S s_16_2: call EDECCR_read(s_16_1)
        let s_16_2: ProductType700c18a878c5601b = EDECCR_read(state, tracer, s_16_1);
        // D s_16_3: write-var ga#4140 <= s_16_2
        fn_state.ga_4140 = s_16_2;
        // D s_16_4: read-var ga#4140.0:struct
        let s_16_4: u32 = fn_state.ga_4140._0;
        // C s_16_5: const #16975u : u32
        let s_16_5: u32 = 16975;
        // D s_16_6: read-reg s_16_5:u8
        let s_16_6: u8 = {
            let value = state.read_register::<u8>(s_16_5 as isize);
            tracer.read_register(s_16_5 as isize, value);
            value
        };
        // D s_16_7: cast zx s_16_6 -> bv
        let s_16_7: Bits = Bits::new(s_16_6 as u128, 2u16);
        // D s_16_8: cast zx s_16_7 -> i
        let s_16_8: i128 = (s_16_7.value() as i128);
        // D s_16_9: cast reint s_16_8 -> i64
        let s_16_9: i64 = (s_16_8 as i64);
        // D s_16_10: cast zx s_16_9 -> i
        let s_16_10: i128 = (i128::try_from(s_16_9).unwrap());
        // D s_16_11: read-var baseshadow#72:i64
        let s_16_11: i64 = fn_state.baseshadow_72;
        // D s_16_12: cast zx s_16_11 -> i
        let s_16_12: i128 = (i128::try_from(s_16_11).unwrap());
        // D s_16_13: add s_16_10 s_16_12
        let s_16_13: i128 = (s_16_10 + s_16_12);
        // D s_16_14: cast reint s_16_13 -> i64
        let s_16_14: i64 = (s_16_13 as i64);
        // D s_16_15: cast zx s_16_14 -> i
        let s_16_15: i128 = (i128::try_from(s_16_14).unwrap());
        // D s_16_16: cast zx s_16_0 -> i
        let s_16_16: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_17: add s_16_15 s_16_16
        let s_16_17: i128 = (s_16_15 + s_16_16);
        // D s_16_18: cast reint s_16_17 -> i64
        let s_16_18: i64 = (s_16_17 as i64);
        // D s_16_19: cast zx s_16_4 -> bv
        let s_16_19: Bits = Bits::new(s_16_4 as u128, 32u16);
        // D s_16_20: cast zx s_16_18 -> i
        let s_16_20: i128 = (i128::try_from(s_16_18).unwrap());
        // C s_16_21: const #1u : u64
        let s_16_21: u64 = 1;
        // D s_16_22: bit-extract s_16_19 s_16_20 s_16_21
        let s_16_22: Bits = (Bits::new(
            ((s_16_19) >> (s_16_20)).value(),
            u16::try_from(s_16_21).unwrap(),
        ));
        // D s_16_23: cast reint s_16_22 -> u8
        let s_16_23: bool = ((s_16_22.value()) != 0);
        // C s_16_24: const #0s : i
        let s_16_24: i128 = 0;
        // C s_16_25: const #0u : u64
        let s_16_25: u64 = 0;
        // D s_16_26: cast zx s_16_23 -> u64
        let s_16_26: u64 = (s_16_23 as u64);
        // C s_16_27: const #1u : u64
        let s_16_27: u64 = 1;
        // D s_16_28: and s_16_26 s_16_27
        let s_16_28: u64 = ((s_16_26) & (s_16_27));
        // D s_16_29: cmp-eq s_16_28 s_16_27
        let s_16_29: bool = ((s_16_28) == (s_16_27));
        // D s_16_30: lsl s_16_26 s_16_24
        let s_16_30: u64 = s_16_26 << s_16_24;
        // D s_16_31: or s_16_25 s_16_30
        let s_16_31: u64 = ((s_16_25) | (s_16_30));
        // D s_16_32: cmpl s_16_30
        let s_16_32: u64 = !s_16_30;
        // D s_16_33: and s_16_25 s_16_32
        let s_16_33: u64 = ((s_16_25) & (s_16_32));
        // D s_16_34: select s_16_29 s_16_31 s_16_33
        let s_16_34: u64 = if s_16_29 { s_16_31 } else { s_16_33 };
        // D s_16_35: cast trunc s_16_34 -> u8
        let s_16_35: bool = ((s_16_34) != 0);
        // C s_16_36: const #() : ()
        let s_16_36: () = ();
        // S s_16_37: call EDECCR_read(s_16_36)
        let s_16_37: ProductType700c18a878c5601b = EDECCR_read(state, tracer, s_16_36);
        // D s_16_38: write-var ga#4147 <= s_16_37
        fn_state.ga_4147 = s_16_37;
        // D s_16_39: read-var ga#4147.0:struct
        let s_16_39: u32 = fn_state.ga_4147._0;
        // C s_16_40: const #16975u : u32
        let s_16_40: u32 = 16975;
        // D s_16_41: read-reg s_16_40:u8
        let s_16_41: u8 = {
            let value = state.read_register::<u8>(s_16_40 as isize);
            tracer.read_register(s_16_40 as isize, value);
            value
        };
        // D s_16_42: cast zx s_16_41 -> bv
        let s_16_42: Bits = Bits::new(s_16_41 as u128, 2u16);
        // D s_16_43: cast zx s_16_42 -> i
        let s_16_43: i128 = (s_16_42.value() as i128);
        // D s_16_44: cast reint s_16_43 -> i64
        let s_16_44: i64 = (s_16_43 as i64);
        // D s_16_45: cast zx s_16_44 -> i
        let s_16_45: i128 = (i128::try_from(s_16_44).unwrap());
        // D s_16_46: read-var baseshadow#72:i64
        let s_16_46: i64 = fn_state.baseshadow_72;
        // D s_16_47: cast zx s_16_46 -> i
        let s_16_47: i128 = (i128::try_from(s_16_46).unwrap());
        // D s_16_48: add s_16_45 s_16_47
        let s_16_48: i128 = (s_16_45 + s_16_47);
        // D s_16_49: cast reint s_16_48 -> i64
        let s_16_49: i64 = (s_16_48 as i64);
        // D s_16_50: cast zx s_16_39 -> bv
        let s_16_50: Bits = Bits::new(s_16_39 as u128, 32u16);
        // D s_16_51: cast zx s_16_49 -> i
        let s_16_51: i128 = (i128::try_from(s_16_49).unwrap());
        // C s_16_52: const #1u : u64
        let s_16_52: u64 = 1;
        // D s_16_53: bit-extract s_16_50 s_16_51 s_16_52
        let s_16_53: Bits = (Bits::new(
            ((s_16_50) >> (s_16_51)).value(),
            u16::try_from(s_16_52).unwrap(),
        ));
        // D s_16_54: cast reint s_16_53 -> u8
        let s_16_54: bool = ((s_16_53.value()) != 0);
        // C s_16_55: const #0s : i
        let s_16_55: i128 = 0;
        // C s_16_56: const #0u : u64
        let s_16_56: u64 = 0;
        // D s_16_57: cast zx s_16_54 -> u64
        let s_16_57: u64 = (s_16_54 as u64);
        // C s_16_58: const #1u : u64
        let s_16_58: u64 = 1;
        // D s_16_59: and s_16_57 s_16_58
        let s_16_59: u64 = ((s_16_57) & (s_16_58));
        // D s_16_60: cmp-eq s_16_59 s_16_58
        let s_16_60: bool = ((s_16_59) == (s_16_58));
        // D s_16_61: lsl s_16_57 s_16_55
        let s_16_61: u64 = s_16_57 << s_16_55;
        // D s_16_62: or s_16_56 s_16_61
        let s_16_62: u64 = ((s_16_56) | (s_16_61));
        // D s_16_63: cmpl s_16_61
        let s_16_63: u64 = !s_16_61;
        // D s_16_64: and s_16_56 s_16_63
        let s_16_64: u64 = ((s_16_56) & (s_16_63));
        // D s_16_65: select s_16_60 s_16_62 s_16_64
        let s_16_65: u64 = if s_16_60 { s_16_62 } else { s_16_64 };
        // D s_16_66: cast trunc s_16_65 -> u8
        let s_16_66: bool = ((s_16_65) != 0);
        // D s_16_67: cast zx s_16_35 -> bv
        let s_16_67: Bits = Bits::new(s_16_35 as u128, 1u16);
        // D s_16_68: cast zx s_16_66 -> bv
        let s_16_68: Bits = Bits::new(s_16_66 as u128, 1u16);
        // D s_16_69: cast reint s_16_67 -> u128
        let s_16_69: u128 = (s_16_67.value() as u128);
        // D s_16_70: size-of s_16_67
        let s_16_70: u16 = s_16_67.length();
        // D s_16_71: cast reint s_16_68 -> u128
        let s_16_71: u128 = (s_16_68.value() as u128);
        // D s_16_72: size-of s_16_68
        let s_16_72: u16 = s_16_68.length();
        // D s_16_73: lsl s_16_69 s_16_72
        let s_16_73: u128 = s_16_69 << s_16_72;
        // D s_16_74: or s_16_73 s_16_71
        let s_16_74: u128 = ((s_16_73) | (s_16_71));
        // D s_16_75: add s_16_70 s_16_72
        let s_16_75: u16 = (s_16_70 + s_16_72);
        // D s_16_76: create-bits s_16_74 s_16_75
        let s_16_76: Bits = Bits::new(s_16_74, s_16_75);
        // D s_16_77: cast reint s_16_76 -> u8
        let s_16_77: u8 = (s_16_76.value() as u8);
        // D s_16_78: write-var ctrlshadow#75 <= s_16_77
        fn_state.ctrlshadow_75 = s_16_77;
        // D s_16_79: read-var ctrlshadow#75:u8
        let s_16_79: u8 = fn_state.ctrlshadow_75;
        // D s_16_80: cast zx s_16_79 -> bv
        let s_16_80: Bits = Bits::new(s_16_79 as u128, 2u16);
        // C s_16_81: const #0u : u8
        let s_16_81: u8 = 0;
        // C s_16_82: cast zx s_16_81 -> bv
        let s_16_82: Bits = Bits::new(s_16_81 as u128, 2u16);
        // D s_16_83: cmp-eq s_16_80 s_16_82
        let s_16_83: bool = ((s_16_80) == (s_16_82));
        // D s_16_84: not s_16_83
        let s_16_84: bool = !s_16_83;
        // N s_16_85: branch s_16_84 b19 b17
        if s_16_84 {
            return block_19(state, tracer, fn_state);
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
        // D s_17_1: write-var halt <= s_17_0
        fn_state.halt = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var ctrlshadow#75:u8
        let s_19_0: u8 = fn_state.ctrlshadow_75;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 2u16);
        // C s_19_2: const #1u : u8
        let s_19_2: u8 = 1;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 2u16);
        // D s_19_4: cmp-eq s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) == (s_19_3));
        // D s_19_5: not s_19_4
        let s_19_5: bool = !s_19_4;
        // N s_19_6: branch s_19_5 b21 b20
        if s_19_5 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var halt <= s_20_0
        fn_state.halt = s_20_0;
        // N s_20_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var ctrlshadow#75:u8
        let s_21_0: u8 = fn_state.ctrlshadow_75;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 2u16);
        // C s_21_2: const #2u : u8
        let s_21_2: u8 = 2;
        // C s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 2u16);
        // D s_21_4: cmp-eq s_21_1 s_21_3
        let s_21_4: bool = ((s_21_1) == (s_21_3));
        // D s_21_5: not s_21_4
        let s_21_5: bool = !s_21_4;
        // N s_21_6: branch s_21_5 b23 b22
        if s_21_5 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var exception_exitshadow#73:u8
        let s_22_0: bool = fn_state.exception_exitshadow_73;
        // C s_22_1: const #1u : u8
        let s_22_1: bool = true;
        // D s_22_2: cmp-eq s_22_0 s_22_1
        let s_22_2: bool = ((s_22_0) == (s_22_1));
        // D s_22_3: write-var halt <= s_22_2
        fn_state.halt = s_22_2;
        // N s_22_4: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var exception_entry:u8
        let s_23_0: bool = fn_state.exception_entry;
        // C s_23_1: const #1u : u8
        let s_23_1: bool = true;
        // D s_23_2: cmp-eq s_23_0 s_23_1
        let s_23_2: bool = ((s_23_0) == (s_23_1));
        // D s_23_3: write-var halt <= s_23_2
        fn_state.halt = s_23_2;
        // N s_23_4: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #4s : i64
        let s_24_0: i64 = 4;
        // D s_24_1: write-var ga#4139 <= s_24_0
        fn_state.ga_4139 = s_24_0;
        // N s_24_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u32
        let s_25_0: u32 = 0;
        // D s_25_1: read-var ss:u32
        let s_25_1: u32 = fn_state.ss;
        // D s_25_2: cmp-eq s_25_0 s_25_1
        let s_25_2: bool = ((s_25_0) == (s_25_1));
        // D s_25_3: not s_25_2
        let s_25_3: bool = !s_25_2;
        // N s_25_4: branch s_25_3 b27 b26
        if s_25_3 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #4s : i64
        let s_26_0: i64 = 4;
        // D s_26_1: write-var base <= s_26_0
        fn_state.base = s_26_0;
        // N s_26_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #2u : u32
        let s_27_0: u32 = 2;
        // D s_27_1: read-var ss:u32
        let s_27_1: u32 = fn_state.ss;
        // D s_27_2: cmp-eq s_27_0 s_27_1
        let s_27_2: bool = ((s_27_0) == (s_27_1));
        // D s_27_3: not s_27_2
        let s_27_3: bool = !s_27_2;
        // N s_27_4: branch s_27_3 b29 b28
        if s_27_3 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #16s : i64
        let s_28_0: i64 = 16;
        // D s_28_1: write-var base <= s_28_0
        fn_state.base = s_28_0;
        // N s_28_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #1u : u32
        let s_29_0: u32 = 1;
        // D s_29_1: read-var ss:u32
        let s_29_1: u32 = fn_state.ss;
        // D s_29_2: cmp-eq s_29_0 s_29_1
        let s_29_2: bool = ((s_29_0) == (s_29_1));
        // D s_29_3: not s_29_2
        let s_29_3: bool = !s_29_2;
        // N s_29_4: branch s_29_3 b31 b30
        if s_29_3 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #0s : i64
        let s_30_0: i64 = 0;
        // D s_30_1: write-var base <= s_30_0
        fn_state.base = s_30_0;
        // N s_30_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_31_0: jump b2
        return block_2(state, tracer, fn_state);
    }
}
