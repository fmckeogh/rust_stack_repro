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
use DISR_write::*;
use AArch32_PhysicalSErrorSyndrome::*;
use ClearPendingPhysicalSError::*;
use IsSynchronizablePhysicalSErrorPending::*;
use Halted::*;
use ELUsingAArch32::*;
use AArch64_PhysicalSErrorSyndrome::*;
use S1TranslationRegime__1::*;
use ExternalDebugInterruptsDisabled::*;
use Zeros::*;
use AArch64_PhysicalSErrorTarget::*;
use Bit::*;
use Mk_DISR_EL1_Type::*;
use Mk_DISR_Type::*;
use common::*;
pub fn AArch64_ESBOperation<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_24367: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        masked: bool,
        intdis: bool,
        gs_24374: bool,
        gs_24373: bool,
        ga_18998: ProductTypea5cc8de4daab131c,
        target_el: u8,
        gs_24372: bool,
        gs_24367: (),
    }
    let fn_state = FunctionState {
        gs_24367,
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
        // S s_0_1: call AArch64_PhysicalSErrorTarget(s_0_0)
        let s_0_1: ProductTypea5cc8de4daab131c = AArch64_PhysicalSErrorTarget(
            state,
            tracer,
            s_0_0,
        );
        // D s_0_2: write-var ga#18998 <= s_0_1
        fn_state.ga_18998 = s_0_1;
        // D s_0_3: read-var ga#18998.0:struct
        let s_0_3: bool = fn_state.ga_18998._0;
        // D s_0_4: read-var ga#18998.1:struct
        let s_0_4: u8 = fn_state.ga_18998._1;
        // D s_0_5: write-var masked <= s_0_3
        fn_state.masked = s_0_3;
        // D s_0_6: write-var target_el <= s_0_4
        fn_state.target_el = s_0_4;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call Halted(s_0_7)
        let s_0_8: bool = Halted(state, tracer, s_0_7);
        // N s_0_9: branch s_0_8 b14 b1
        if s_0_8 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var target_el:u8
        let s_1_0: u8 = fn_state.target_el;
        // D s_1_1: call ExternalDebugInterruptsDisabled(s_1_0)
        let s_1_1: bool = ExternalDebugInterruptsDisabled(state, tracer, s_1_0);
        // D s_1_2: write-var gs#24372 <= s_1_1
        fn_state.gs_24372 = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#24372:u8
        let s_2_0: bool = fn_state.gs_24372;
        // D s_2_1: write-var intdis <= s_2_0
        fn_state.intdis = s_2_0;
        // D s_2_2: read-var masked:u8
        let s_2_2: bool = fn_state.masked;
        // N s_2_3: branch s_2_2 b13 b3
        if s_2_2 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var intdis:u8
        let s_3_0: bool = fn_state.intdis;
        // D s_3_1: write-var gs#24373 <= s_3_0
        fn_state.gs_24373 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#24373:u8
        let s_4_0: bool = fn_state.gs_24373;
        // N s_4_1: branch s_4_0 b12 b5
        if s_4_0 {
            return block_12(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#24374 <= s_5_0
        fn_state.gs_24374 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#24374:u8
        let s_6_0: bool = fn_state.gs_24374;
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
        // S s_8_1: call S1TranslationRegime__1(s_8_0)
        let s_8_1: u8 = S1TranslationRegime__1(state, tracer, s_8_0);
        // S s_8_2: call ELUsingAArch32(s_8_1)
        let s_8_2: bool = ELUsingAArch32(state, tracer, s_8_1);
        // N s_8_3: branch s_8_2 b11 b9
        if s_8_2 {
            return block_11(state, tracer, fn_state);
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
        // C s_9_1: const #64s : i
        let s_9_1: i128 = 64;
        // S s_9_2: call Zeros(s_9_1)
        let s_9_2: Bits = Zeros(state, tracer, s_9_1);
        // S s_9_3: cast reint s_9_2 -> u64
        let s_9_3: u64 = (s_9_2.value() as u64);
        // C s_9_4: const #1u : u8
        let s_9_4: bool = true;
        // S s_9_5: call Bit(s_9_4)
        let s_9_5: bool = Bit(state, tracer, s_9_4);
        // C s_9_6: const #31s : i
        let s_9_6: i128 = 31;
        // S s_9_7: cast zx s_9_3 -> bv
        let s_9_7: Bits = Bits::new(s_9_3 as u128, 64u16);
        // C s_9_8: const #1u : u64
        let s_9_8: u64 = 1;
        // D s_9_9: bit-insert s_9_7 s_9_7 s_9_6 s_9_8
        let s_9_9: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_9_8 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_9_7.length(),
            );
            (s_9_7 & mask) | (s_9_7 << s_9_6)
        };
        // D s_9_10: cast reint s_9_9 -> u64
        let s_9_10: u64 = (s_9_9.value() as u64);
        // S s_9_11: call AArch64_PhysicalSErrorSyndrome(s_9_0)
        let s_9_11: u32 = AArch64_PhysicalSErrorSyndrome(state, tracer, s_9_0);
        // C s_9_12: const #0s : i
        let s_9_12: i128 = 0;
        // D s_9_13: cast zx s_9_10 -> bv
        let s_9_13: Bits = Bits::new(s_9_10 as u128, 64u16);
        // S s_9_14: cast zx s_9_11 -> bv
        let s_9_14: Bits = Bits::new(s_9_11 as u128, 25u16);
        // C s_9_15: const #24s : i
        let s_9_15: i128 = 24;
        // C s_9_16: const #1u : u64
        let s_9_16: u64 = 1;
        // C s_9_17: cast zx s_9_16 -> bv
        let s_9_17: Bits = Bits::new(s_9_16 as u128, 64u16);
        // C s_9_18: lsl s_9_17 s_9_15
        let s_9_18: Bits = s_9_17 << s_9_15;
        // C s_9_19: sub s_9_18 s_9_17
        let s_9_19: Bits = ((s_9_18) - (s_9_17));
        // S s_9_20: and s_9_14 s_9_19
        let s_9_20: Bits = ((s_9_14) & (s_9_19));
        // S s_9_21: lsl s_9_20 s_9_12
        let s_9_21: Bits = s_9_20 << s_9_12;
        // C s_9_22: lsl s_9_19 s_9_12
        let s_9_22: Bits = s_9_19 << s_9_12;
        // C s_9_23: cmpl s_9_22
        let s_9_23: Bits = !s_9_22;
        // D s_9_24: and s_9_13 s_9_23
        let s_9_24: Bits = ((s_9_13) & (s_9_23));
        // D s_9_25: or s_9_24 s_9_21
        let s_9_25: Bits = ((s_9_24) | (s_9_21));
        // D s_9_26: cast reint s_9_25 -> u64
        let s_9_26: u64 = (s_9_25.value() as u64);
        // D s_9_27: call Mk_DISR_EL1_Type(s_9_26)
        let s_9_27: ProductType5c790c8ef59cc8b2 = Mk_DISR_EL1_Type(
            state,
            tracer,
            s_9_26,
        );
        // C s_9_28: const #13632u : u32
        let s_9_28: u32 = 13632;
        // N s_9_29: write-reg s_9_28 <= s_9_27
        let s_9_29: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_9_28 as isize, s_9_27);
            tracer.write_register(s_9_28 as isize, s_9_27);
        };
        // N s_9_30: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call ClearPendingPhysicalSError(s_10_0)
        let s_10_1: () = ClearPendingPhysicalSError(state, tracer, s_10_0);
        // N s_10_2: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #32s : i
        let s_11_0: i128 = 32;
        // S s_11_1: call Zeros(s_11_0)
        let s_11_1: Bits = Zeros(state, tracer, s_11_0);
        // S s_11_2: cast reint s_11_1 -> u32
        let s_11_2: u32 = (s_11_1.value() as u32);
        // C s_11_3: const #1u : u8
        let s_11_3: bool = true;
        // S s_11_4: call Bit(s_11_3)
        let s_11_4: bool = Bit(state, tracer, s_11_3);
        // C s_11_5: const #31s : i
        let s_11_5: i128 = 31;
        // S s_11_6: cast zx s_11_2 -> bv
        let s_11_6: Bits = Bits::new(s_11_2 as u128, 32u16);
        // C s_11_7: const #1u : u64
        let s_11_7: u64 = 1;
        // D s_11_8: bit-insert s_11_6 s_11_6 s_11_5 s_11_7
        let s_11_8: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_11_7 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_11_6.length(),
            );
            (s_11_6 & mask) | (s_11_6 << s_11_5)
        };
        // D s_11_9: cast reint s_11_8 -> u32
        let s_11_9: u32 = (s_11_8.value() as u32);
        // C s_11_10: const #() : ()
        let s_11_10: () = ();
        // S s_11_11: call AArch32_PhysicalSErrorSyndrome(s_11_10)
        let s_11_11: u16 = AArch32_PhysicalSErrorSyndrome(state, tracer, s_11_10);
        // C s_11_12: const #0s : i
        let s_11_12: i128 = 0;
        // D s_11_13: cast zx s_11_9 -> bv
        let s_11_13: Bits = Bits::new(s_11_9 as u128, 32u16);
        // S s_11_14: cast zx s_11_11 -> bv
        let s_11_14: Bits = Bits::new(s_11_11 as u128, 16u16);
        // C s_11_15: const #15s : i
        let s_11_15: i128 = 15;
        // C s_11_16: const #1u : u64
        let s_11_16: u64 = 1;
        // C s_11_17: cast zx s_11_16 -> bv
        let s_11_17: Bits = Bits::new(s_11_16 as u128, 64u16);
        // C s_11_18: lsl s_11_17 s_11_15
        let s_11_18: Bits = s_11_17 << s_11_15;
        // C s_11_19: sub s_11_18 s_11_17
        let s_11_19: Bits = ((s_11_18) - (s_11_17));
        // S s_11_20: and s_11_14 s_11_19
        let s_11_20: Bits = ((s_11_14) & (s_11_19));
        // S s_11_21: lsl s_11_20 s_11_12
        let s_11_21: Bits = s_11_20 << s_11_12;
        // C s_11_22: lsl s_11_19 s_11_12
        let s_11_22: Bits = s_11_19 << s_11_12;
        // C s_11_23: cmpl s_11_22
        let s_11_23: Bits = !s_11_22;
        // D s_11_24: and s_11_13 s_11_23
        let s_11_24: Bits = ((s_11_13) & (s_11_23));
        // D s_11_25: or s_11_24 s_11_21
        let s_11_25: Bits = ((s_11_24) | (s_11_21));
        // D s_11_26: cast reint s_11_25 -> u32
        let s_11_26: u32 = (s_11_25.value() as u32);
        // D s_11_27: call Mk_DISR_Type(s_11_26)
        let s_11_27: ProductType700c18a878c5601b = Mk_DISR_Type(state, tracer, s_11_26);
        // D s_11_28: call DISR_write(s_11_27)
        let s_11_28: () = DISR_write(state, tracer, s_11_27);
        // N s_11_29: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call IsSynchronizablePhysicalSErrorPending(s_12_0)
        let s_12_1: bool = IsSynchronizablePhysicalSErrorPending(state, tracer, s_12_0);
        // D s_12_2: write-var gs#24374 <= s_12_1
        fn_state.gs_24374 = s_12_1;
        // N s_12_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#24373 <= s_13_0
        fn_state.gs_24373 = s_13_0;
        // N s_13_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#24372 <= s_14_0
        fn_state.gs_24372 = s_14_0;
        // N s_14_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
