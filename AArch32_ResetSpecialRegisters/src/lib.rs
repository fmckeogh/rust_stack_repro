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
use DLR_write::*;
use DSPSR_write::*;
use Mk_DSPSR_Type::*;
use SPSR_svc_write::*;
use Mk_SPSR_mon_Type::*;
use Mk_SPSR_svc_Type::*;
use SPSR_svc_read::*;
use SPSR_hyp_write::*;
use ELR_hyp_write::*;
use Mk_SPSR_hyp_Type::*;
use common::*;
pub fn AArch32_ResetSpecialRegisters<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_31331: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_24397: ProductType700c18a878c5601b,
        gs_31331: (),
    }
    let fn_state = FunctionState {
        gs_31331,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #32s : i64
        let s_0_0: i64 = 32;
        // C s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // S s_0_2: call __UNKNOWN_bits(s_0_1)
        let s_0_2: Bits = u__UNKNOWN_bits(state, tracer, s_0_1);
        // C s_0_3: const #15536u : u32
        let s_0_3: u32 = 15536;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // C s_0_5: const #15536u : u32
        let s_0_5: u32 = 15536;
        // N s_0_6: write-reg s_0_5 <= s_0_4
        let s_0_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_5 as isize, s_0_4);
            tracer.write_register(s_0_5 as isize, s_0_4);
        };
        // C s_0_7: const #32s : i64
        let s_0_7: i64 = 32;
        // C s_0_8: cast zx s_0_7 -> i
        let s_0_8: i128 = (i128::try_from(s_0_7).unwrap());
        // S s_0_9: call __UNKNOWN_bits(s_0_8)
        let s_0_9: Bits = u__UNKNOWN_bits(state, tracer, s_0_8);
        // C s_0_10: const #91016u : u32
        let s_0_10: u32 = 91016;
        // D s_0_11: read-reg s_0_10:struct
        let s_0_11: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_10 as isize);
            tracer.read_register(s_0_10 as isize, value);
            value
        };
        // C s_0_12: const #91016u : u32
        let s_0_12: u32 = 91016;
        // N s_0_13: write-reg s_0_12 <= s_0_11
        let s_0_13: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_12 as isize, s_0_11);
            tracer.write_register(s_0_12 as isize, s_0_11);
        };
        // C s_0_14: const #() : ()
        let s_0_14: () = ();
        // S s_0_15: call SPSR_svc_read(s_0_14)
        let s_0_15: ProductType700c18a878c5601b = SPSR_svc_read(state, tracer, s_0_14);
        // D s_0_16: write-var ga#24397 <= s_0_15
        fn_state.ga_24397 = s_0_15;
        // D s_0_17: read-var ga#24397.0:struct
        let s_0_17: u32 = fn_state.ga_24397._0;
        // C s_0_18: const #32s : i64
        let s_0_18: i64 = 32;
        // C s_0_19: cast zx s_0_18 -> i
        let s_0_19: i128 = (i128::try_from(s_0_18).unwrap());
        // S s_0_20: call __UNKNOWN_bits(s_0_19)
        let s_0_20: Bits = u__UNKNOWN_bits(state, tracer, s_0_19);
        // S s_0_21: cast reint s_0_20 -> u32
        let s_0_21: u32 = (s_0_20.value() as u32);
        // C s_0_22: const #0s : i
        let s_0_22: i128 = 0;
        // D s_0_23: cast zx s_0_17 -> bv
        let s_0_23: Bits = Bits::new(s_0_17 as u128, 32u16);
        // S s_0_24: cast zx s_0_21 -> bv
        let s_0_24: Bits = Bits::new(s_0_21 as u128, 32u16);
        // C s_0_25: const #31s : i
        let s_0_25: i128 = 31;
        // C s_0_26: const #1u : u64
        let s_0_26: u64 = 1;
        // C s_0_27: cast zx s_0_26 -> bv
        let s_0_27: Bits = Bits::new(s_0_26 as u128, 64u16);
        // C s_0_28: lsl s_0_27 s_0_25
        let s_0_28: Bits = s_0_27 << s_0_25;
        // C s_0_29: sub s_0_28 s_0_27
        let s_0_29: Bits = ((s_0_28) - (s_0_27));
        // S s_0_30: and s_0_24 s_0_29
        let s_0_30: Bits = ((s_0_24) & (s_0_29));
        // S s_0_31: lsl s_0_30 s_0_22
        let s_0_31: Bits = s_0_30 << s_0_22;
        // C s_0_32: lsl s_0_29 s_0_22
        let s_0_32: Bits = s_0_29 << s_0_22;
        // C s_0_33: cmpl s_0_32
        let s_0_33: Bits = !s_0_32;
        // D s_0_34: and s_0_23 s_0_33
        let s_0_34: Bits = ((s_0_23) & (s_0_33));
        // D s_0_35: or s_0_34 s_0_31
        let s_0_35: Bits = ((s_0_34) | (s_0_31));
        // D s_0_36: cast reint s_0_35 -> u32
        let s_0_36: u32 = (s_0_35.value() as u32);
        // D s_0_37: call Mk_SPSR_svc_Type(s_0_36)
        let s_0_37: ProductType700c18a878c5601b = Mk_SPSR_svc_Type(
            state,
            tracer,
            s_0_36,
        );
        // D s_0_38: call SPSR_svc_write(s_0_37)
        let s_0_38: () = SPSR_svc_write(state, tracer, s_0_37);
        // C s_0_39: const #32s : i64
        let s_0_39: i64 = 32;
        // C s_0_40: cast zx s_0_39 -> i
        let s_0_40: i128 = (i128::try_from(s_0_39).unwrap());
        // S s_0_41: call __UNKNOWN_bits(s_0_40)
        let s_0_41: Bits = u__UNKNOWN_bits(state, tracer, s_0_40);
        // C s_0_42: const #20032u : u32
        let s_0_42: u32 = 20032;
        // D s_0_43: read-reg s_0_42:struct
        let s_0_43: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_42 as isize);
            tracer.read_register(s_0_42 as isize, value);
            value
        };
        // C s_0_44: const #20032u : u32
        let s_0_44: u32 = 20032;
        // N s_0_45: write-reg s_0_44 <= s_0_43
        let s_0_45: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_44 as isize, s_0_43);
            tracer.write_register(s_0_44 as isize, s_0_43);
        };
        // C s_0_46: const #32s : i64
        let s_0_46: i64 = 32;
        // C s_0_47: cast zx s_0_46 -> i
        let s_0_47: i128 = (i128::try_from(s_0_46).unwrap());
        // S s_0_48: call __UNKNOWN_bits(s_0_47)
        let s_0_48: Bits = u__UNKNOWN_bits(state, tracer, s_0_47);
        // C s_0_49: const #18424u : u32
        let s_0_49: u32 = 18424;
        // D s_0_50: read-reg s_0_49:struct
        let s_0_50: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_49 as isize);
            tracer.read_register(s_0_49 as isize, value);
            value
        };
        // C s_0_51: const #18424u : u32
        let s_0_51: u32 = 18424;
        // N s_0_52: write-reg s_0_51 <= s_0_50
        let s_0_52: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_51 as isize, s_0_50);
            tracer.write_register(s_0_51 as isize, s_0_50);
        };
        // C s_0_53: const #432u : u32
        let s_0_53: u32 = 432;
        // D s_0_54: read-reg s_0_53:u8
        let s_0_54: u8 = {
            let value = state.read_register::<u8>(s_0_53 as isize);
            tracer.read_register(s_0_53 as isize, value);
            value
        };
        // C s_0_55: const #2u : u8
        let s_0_55: u8 = 2;
        // D s_0_56: cmp-lt s_0_54 s_0_55
        let s_0_56: bool = ((s_0_54) < (s_0_55));
        // N s_0_57: branch s_0_56 b6 b1
        if s_0_56 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #424u : u32
        let s_2_0: u32 = 424;
        // D s_2_1: read-reg s_2_0:u8
        let s_2_1: u8 = {
            let value = state.read_register::<u8>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // C s_2_2: const #2u : u8
        let s_2_2: u8 = 2;
        // D s_2_3: cmp-lt s_2_1 s_2_2
        let s_2_3: bool = ((s_2_1) < (s_2_2));
        // N s_2_4: branch s_2_3 b5 b3
        if s_2_3 {
            return block_5(state, tracer, fn_state);
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
        // C s_4_0: const #32s : i64
        let s_4_0: i64 = 32;
        // C s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // S s_4_2: call __UNKNOWN_bits(s_4_1)
        let s_4_2: Bits = u__UNKNOWN_bits(state, tracer, s_4_1);
        // S s_4_3: cast reint s_4_2 -> u32
        let s_4_3: u32 = (s_4_2.value() as u32);
        // S s_4_4: call DLR_write(s_4_3)
        let s_4_4: () = DLR_write(state, tracer, s_4_3);
        // C s_4_5: const #32s : i64
        let s_4_5: i64 = 32;
        // C s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // S s_4_7: call __UNKNOWN_bits(s_4_6)
        let s_4_7: Bits = u__UNKNOWN_bits(state, tracer, s_4_6);
        // S s_4_8: cast reint s_4_7 -> u32
        let s_4_8: u32 = (s_4_7.value() as u32);
        // S s_4_9: call Mk_DSPSR_Type(s_4_8)
        let s_4_9: ProductType700c18a878c5601b = Mk_DSPSR_Type(state, tracer, s_4_8);
        // S s_4_10: call DSPSR_write(s_4_9)
        let s_4_10: () = DSPSR_write(state, tracer, s_4_9);
        // N s_4_11: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #32s : i64
        let s_5_0: i64 = 32;
        // C s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // S s_5_2: call __UNKNOWN_bits(s_5_1)
        let s_5_2: Bits = u__UNKNOWN_bits(state, tracer, s_5_1);
        // S s_5_3: cast reint s_5_2 -> u32
        let s_5_3: u32 = (s_5_2.value() as u32);
        // S s_5_4: call Mk_SPSR_mon_Type(s_5_3)
        let s_5_4: ProductType700c18a878c5601b = Mk_SPSR_mon_Type(state, tracer, s_5_3);
        // C s_5_5: const #21136u : u32
        let s_5_5: u32 = 21136;
        // N s_5_6: write-reg s_5_5 <= s_5_4
        let s_5_6: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_5_5 as isize, s_5_4);
            tracer.write_register(s_5_5 as isize, s_5_4);
        };
        // N s_5_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #32s : i64
        let s_6_0: i64 = 32;
        // C s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // S s_6_2: call __UNKNOWN_bits(s_6_1)
        let s_6_2: Bits = u__UNKNOWN_bits(state, tracer, s_6_1);
        // S s_6_3: cast reint s_6_2 -> u32
        let s_6_3: u32 = (s_6_2.value() as u32);
        // S s_6_4: call Mk_SPSR_hyp_Type(s_6_3)
        let s_6_4: ProductType700c18a878c5601b = Mk_SPSR_hyp_Type(state, tracer, s_6_3);
        // S s_6_5: call SPSR_hyp_write(s_6_4)
        let s_6_5: () = SPSR_hyp_write(state, tracer, s_6_4);
        // C s_6_6: const #32s : i64
        let s_6_6: i64 = 32;
        // C s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (i128::try_from(s_6_6).unwrap());
        // S s_6_8: call __UNKNOWN_bits(s_6_7)
        let s_6_8: Bits = u__UNKNOWN_bits(state, tracer, s_6_7);
        // S s_6_9: cast reint s_6_8 -> u32
        let s_6_9: u32 = (s_6_8.value() as u32);
        // S s_6_10: call ELR_hyp_write(s_6_9)
        let s_6_10: () = ELR_hyp_write(state, tracer, s_6_9);
        // N s_6_11: jump b2
        return block_2(state, tracer, fn_state);
    }
}
