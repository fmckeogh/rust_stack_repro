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
use AArch64_GetS1TTWParams::*;
use CurrentSecurityState::*;
use AArch64_PACEffectiveTxSZ::*;
use TranslationRegime::*;
use replicate_bits_borealis_internal::*;
use common::*;
pub fn CalculateBottomPACBit<T: Tracer>(
    state: &mut State,
    tracer: &T,
    top_bit: bool,
) -> i128 {
    #[derive(Default)]
    struct FunctionState {
        top_bit: bool,
    }
    let fn_state = FunctionState {
        top_bit,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call TranslationRegime(s_0_1)
        let s_0_2: u32 = TranslationRegime(state, tracer, s_0_1);
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call CurrentSecurityState(s_0_3)
        let s_0_4: u32 = CurrentSecurityState(state, tracer, s_0_3);
        // D s_0_5: read-var top_bit:u8
        let s_0_5: bool = fn_state.top_bit;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 1u16);
        // C s_0_7: const #64u : u64
        let s_0_7: u64 = 64;
        // D s_0_8: call replicate_bits_borealis_internal(s_0_6, s_0_7)
        let s_0_8: Bits = replicate_bits_borealis_internal(state, tracer, s_0_6, s_0_7);
        // D s_0_9: cast reint s_0_8 -> u64
        let s_0_9: u64 = (s_0_8.value() as u64);
        // D s_0_10: call AArch64_GetS1TTWParams(s_0_2, s_0_4, s_0_9)
        let s_0_10: ProductTypeef284266e139aee2 = AArch64_GetS1TTWParams(
            state,
            tracer,
            s_0_2,
            s_0_4,
            s_0_9,
        );
        // D s_0_11: call AArch64_PACEffectiveTxSZ(s_0_2, s_0_10)
        let s_0_11: u8 = AArch64_PACEffectiveTxSZ(state, tracer, s_0_2, s_0_10);
        // D s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 6u16);
        // D s_0_13: cast zx s_0_12 -> i
        let s_0_13: i128 = (s_0_12.value() as i128);
        // D s_0_14: cast reint s_0_13 -> i64
        let s_0_14: i64 = (s_0_13 as i64);
        // C s_0_15: const #64s : i
        let s_0_15: i128 = 64;
        // D s_0_16: cast zx s_0_14 -> i
        let s_0_16: i128 = (i128::try_from(s_0_14).unwrap());
        // D s_0_17: sub s_0_15 s_0_16
        let s_0_17: i128 = ((s_0_15) - (s_0_16));
        // D s_0_18: cast reint s_0_17 -> i64
        let s_0_18: i64 = (s_0_17 as i64);
        // D s_0_19: cast zx s_0_18 -> i
        let s_0_19: i128 = (i128::try_from(s_0_18).unwrap());
        // N s_0_20: return s_0_19
        return s_0_19;
    }
}
